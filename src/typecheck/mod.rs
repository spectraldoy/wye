mod util;

use crate::parse::ast::{Statement, TypeExpression, Expression};
use std::collections::{HashMap, HashSet};

pub struct TypeChecker<'a> {
    // program to type check
    program: &'a Vec<Statement>,
    // declared variable -> (type_variables, type)
    context: HashMap<String, (HashSet<String>, TypeExpression)>,
    // map of declared typename to type variables
    declared_typenames: HashMap<String, HashSet<String>>,
    // declared type variant -> type name, field type
    type_variants: HashMap<String, (String, Option<TypeExpression>)>,
    // next available integer to use to generate type variables
    next_k: usize
}

impl<'a> TypeChecker<'a> {
    pub fn new(program: &'a Vec<Statement>) -> Result<Self, String> {        
        // first we gather the declared types and assign initial types
        // to the let statements
        let mut context: HashMap<String, (HashSet<String>, TypeExpression)> = HashMap::new();
        let mut type_variants: HashMap<String, (String, Option<TypeExpression>)> = HashMap::new();
        let mut declared_typenames: HashMap<String, HashSet<String>> = HashMap::new();
        let mut next_k: usize = 0;

        fn collect_functype(types: &[TypeExpression]) -> TypeExpression {
            if types.len() == 1 {
                return types[0].clone();
            }

            let rest_type = collect_functype(&types[1..]);
            TypeExpression::FunctionType(
                Box::new(types[0].clone()),
                Box::new(rest_type)
            )
        }

        for stmt in program {
            match stmt {
                Statement::UntypedLet(ids, _) => {
                    let varname = &ids[0];
                    if context.contains_key(varname) {
                        return Err(format!("Redefinition of variable {}", varname));
                    }
                    if type_variants.contains_key(varname) {
                        return Err(format!(
                            "Trying to declare variable {} when it is already declared as a type variant",
                            varname
                        ));
                    }
                    // guess a type for each id
                    let (types, type_variables): (Vec<TypeExpression>, HashSet<String>) = ids
                        .iter()
                        .map(|_| {
                            let typevar_name = util::next_typevar_name(&mut next_k);
                            (TypeExpression::TypeVariable(typevar_name.clone()), typevar_name)
                        })
                        .unzip();

                    let id_type = collect_functype(&types[..]);
                    // every type is quantified over
                    context.insert(varname.clone(), (type_variables, id_type));
                },
                Statement::TypedLet(varname, out_type, args, _) => {
                    if context.contains_key(varname) {
                        return Err(format!("Redefinition of variable {}", varname));
                    }
                    if type_variants.contains_key(varname) {
                        return Err(format!(
                            "Trying to declare variable {} when it is already declared as a type variant",
                            varname
                        ));
                    }
                    // the types are already specified, we just need to collect them properly
                    let mut types: Vec<TypeExpression> = vec![];
                    let type_variables: HashSet<String> = args
                        .iter()
                        .filter_map(|(_, typ)| {
                            types.push(typ.clone());
                            match typ {
                                TypeExpression::TypeVariable(typevar) => Some(typevar.clone()),
                                _ => None
                            }
                        })
                        .collect::<_>();

                    types.push(out_type.clone());
                    let id_type = collect_functype(&types[..]);
                    context.insert(varname.clone(), (type_variables, id_type));
                },
                Statement::TypeDeclaration(typename, typevars, variants) => {
                    if declared_typenames.contains_key(typename) {
                        return Err(format!("Redefinition of type {}", typename));
                    }

                    // for each variant, make sure the variant has not been declared before
                    for (variant_name, field) in variants {
                        if type_variants.contains_key(variant_name) {
                            return Err(format!(
                                "Trying to declare {} as a variant of type {} when it is already a variant of type {}",
                                variant_name,
                                typename,
                                type_variants.get(variant_name).unwrap().0
                            ));
                        }
                        // if we reach here, we can safely map to the new type
                        type_variants.insert(variant_name.clone(), (typename.clone(), field.clone()));
                    }

                    let mut typevarset: HashSet<String> = HashSet::new();
                    for typevarname in typevars {
                        if typevarset.contains(typevarname) {
                            return Err(format!(
                                "Duplicate type variable {} in definition of type {}",
                                typevarname,
                                typename
                            ));
                        }
                        typevarset.insert(typevarname.clone());
                    }

                    // add the new typename to declared typenames
                    declared_typenames.insert(typename.clone(), typevarset);
                }
            }
        }

        fn type_check_variant_fields(
            type_expression: &TypeExpression,
            type_vars_in_scope: &HashSet<String>,
            declared_typenames: &HashMap<String, HashSet<String>>
        ) -> Result<(), String> {
            match type_expression {
                TypeExpression::DeclaredType(decl_type, type_args) => {
                    if !declared_typenames.contains_key(decl_type) {
                        return Err(format!("Type {} is not declared", decl_type));
                    }

                    let expected_num_type_vars = declared_typenames.get(decl_type).unwrap().len();
                    let actual_num_type_vars = type_args.len();
                    if expected_num_type_vars != actual_num_type_vars {
                        Err(format!(
                            "Expected {} type arguments to declared type {}, got {}",
                            expected_num_type_vars,
                            decl_type,
                            actual_num_type_vars
                        ))
                    } else {
                        type_args
                        .iter()
                        .map(|ta| type_check_variant_fields(ta, type_vars_in_scope, declared_typenames))
                        .fold(Ok(()), |acc, el| acc.and(el))
                    }
                },
                TypeExpression::ListType(te) => {
                    type_check_variant_fields(te, type_vars_in_scope, declared_typenames)
                },
                TypeExpression::TupleType(tup_types) => {
                    tup_types
                    .iter()
                    .map(|t| type_check_variant_fields(t, type_vars_in_scope, declared_typenames))
                    .fold(Ok(()), |acc, el| acc.and(el))
                },
                TypeExpression::TypeVariable(tv) => {
                    if type_vars_in_scope.contains(tv) {
                        Ok(())
                    } else {
                        Err(format!("Type variable {} not in scope", tv))
                    }
                },
                TypeExpression::FunctionType(te_func, te_arg) => {
                    type_check_variant_fields(te_func, type_vars_in_scope, declared_typenames)
                    .and(type_check_variant_fields(te_arg, type_vars_in_scope, declared_typenames))
                },
                // int, float, string are all ok
                _ => Ok(())
            }
        }

        // TODO: the type variables that are added for the untyped lets are not the same as
        // type variables that are used in typed lets and type declarations. The former are primed
        // for unification as part of the type inference algorithm, whereas the latter are meant to
        // be as general as possible.

        // now that the description of every type in the program is known,
        // vet that the fields of declared types do not contradict any type functions
        // that is, that the fields of variants all are declared to be valid types
        for (_, (typename, field_type)) in &type_variants {
            match field_type {
                None => {},
                Some(te) => {
                    let type_vars_in_scope = declared_typenames.get(typename).unwrap();
                    let res = type_check_variant_fields(te, type_vars_in_scope, &declared_typenames);
                    if let Err(e) = res {
                        return Err(e);
                    }
                }
            }
        }

        Ok(Self {
            program,
            context,
            declared_typenames,
            type_variants,
            next_k
        })
    }

    fn x() {

    }
}

impl<'a> TypeChecker<'a> {
    pub fn type_check_program(&self) -> Result<(), String> {
        for stmt in self.program {
            match stmt {
                Statement::UntypedLet(ids, expr) =>
                    self.type_check_untyped_let(ids, expr),
                Statement::TypedLet(id, typ, args, expr) => {
                    Ok(())
                },
                Statement::TypeDeclaration(type_id, type_vars, variants) => {
                    Ok(())
                }
            }?
        }
        Ok(())
    }

    fn type_check_untyped_let(&self, id: &Vec<String>, expr: &Expression) -> Result<(), String> {
        // need to generate a new type variable
        // also we have a function type for every identifier we have as an argument
        let initial_type = TypeExpression::TypeVariable(String::from("a"));
        // but in order for this to happen, the arguments to the function need to be added
        // to the context available in which to type check the expression
        // so we need to create a new context here and pass that into the expression type checker
        let t = self.type_check_expression(expr, &self.context);

        Ok(())
    }

    fn type_check_expression(&self, expr: &Expression, context: &HashMap<String, (HashSet<String>, TypeExpression)>) -> Result<TypeExpression, String> {
        match expr {
            Expression::IntegerLiteral(_) => Ok(TypeExpression::IntType),
            Expression::FloatLiteral(_) => Ok(TypeExpression::FloatType),
            Expression::StringLiteral(_) => Ok(TypeExpression::StringType),
            Expression::Identifier(x) => unimplemented!(),
            _ => unimplemented!()
        }
    }
}
