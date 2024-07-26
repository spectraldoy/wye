mod util;

use crate::parse::ast::{Statement, TypeExpression, Expression};
use std::collections::{HashMap, HashSet};

pub struct TypeChecker<'a> {
    // program to type check
    program: &'a Vec<Statement>,
    // declared variable -> (universally quantified types, type)
    context: HashMap<String, (HashSet<String>, TypeExpression)>,
    // map of declared typename to type variables
    declared_typenames: HashMap<String, HashSet<String>>,
    // declared type variant -> type name, field type
    type_variants: HashMap<String, (String, Option<TypeExpression>)>,
    // next available integer to use to generate type variables
    next_k: usize
}

// Construction impl
impl<'a> TypeChecker<'a> {
    pub fn new(program: &'a Vec<Statement>) -> Result<Self, String> {        
        // first we gather the declared types and assign initial types
        // to the let statements
        let mut context: HashMap<String, (HashSet<String>, TypeExpression)> = HashMap::new();
        let mut type_variants: HashMap<String, (String, Option<TypeExpression>)> = HashMap::new();
        let mut declared_typenames: HashMap<String, HashSet<String>> = HashMap::new();
        let mut next_k: usize = 0;

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
                    let types: Vec<TypeExpression> = ids
                        .iter()
                        .map(|_| {
                            let inftype_name = util::inftype_name(next_k);
                            next_k += 1;
                            TypeExpression::InferableType(inftype_name)
                        })
                        .collect::<_>();
                    
                    let id_type = util::collect_functype(&types[..]);
                    
                    // we don't actually yet know which argument types are forall types
                    let univ_types: HashSet<String> = HashSet::new();
                    context.insert(varname.clone(), (univ_types, id_type));
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
                    // This map filters out the universal types, and pushes all types into
                    // the above types vector
                    let univ_types: HashSet<String> = args
                        .iter()
                        .filter_map(|(_, typ)| {
                            types.push(typ.clone());
                            match typ {
                                TypeExpression::UniversalType(univ_type) => Some(univ_type.clone()),
                                _ => None
                            }
                        })
                        .collect::<_>();

                    types.push(out_type.clone());
                    let id_type = util::collect_functype(&types[..]);
                    context.insert(varname.clone(), (univ_types, id_type));
                },
                Statement::TypeDeclaration(typename, univ_types, variants) => {
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

                    let mut univ_type_set: HashSet<String> = HashSet::new();
                    for univ_typename in univ_types {
                        if univ_type_set.contains(univ_typename) {
                            return Err(format!(
                                "Duplicate type variable {} in definition of type {}",
                                univ_typename,
                                typename
                            ));
                        }
                        univ_type_set.insert(univ_typename.clone());
                    }

                    // add the new typename to declared typenames
                    declared_typenames.insert(typename.clone(), univ_type_set);
                }
            }
        }

        // now that the description of every type in the program is known,
        // vet that the fields of declared types do not contradict any type functions
        // that is, that the fields of variants all are declared to be valid types
        for (_, (typename, field_type)) in &type_variants {
            match field_type {
                None => {},
                Some(te) => {
                    let univ_types_in_scope = declared_typenames.get(typename).unwrap();
                    let res = TypeChecker::check_type_expr_in_context(te, univ_types_in_scope, &declared_typenames);
                    if let Err(e) = res {
                        return Err(e);
                    }
                }
            }
        }

        // also for every typed let in the context, we can ensure that the specified types
        // have the correct number of type arguments, and otherwise make sense
        for (_, (univ_types_in_scope, out_type)) in context.iter() {
            let res = TypeChecker::check_type_expr_in_context(out_type, univ_types_in_scope, &declared_typenames);
            if res.is_err() {
                return Err(res.err().unwrap());
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
    
    // Ensure that variant fields have reasonable arguments
    fn check_type_expr_in_context(
        type_expression: &TypeExpression,
        univ_types_in_scope: &HashSet<String>,
        declared_typenames: &HashMap<String, HashSet<String>>
    ) -> Result<(), String> {
        match type_expression {
            TypeExpression::DeclaredType(decl_type, type_args) => {
                if !declared_typenames.contains_key(decl_type) {
                    return Err(format!("Type {} is not declared", decl_type));
                }

                let expected_num_uvars = declared_typenames.get(decl_type).unwrap().len();
                let actual_num_uvars = type_args.len();
                if expected_num_uvars != actual_num_uvars {
                    Err(format!(
                        "Expected {} type arguments to declared type {}, got {}",
                        expected_num_uvars,
                        decl_type,
                        actual_num_uvars
                    ))
                } else {
                    type_args
                    .iter()
                    .map(|ta| TypeChecker::check_type_expr_in_context(ta, univ_types_in_scope, declared_typenames))
                    .fold(Ok(()), |acc, el| acc.and(el))
                }
            },
            TypeExpression::ListType(te) => {
                TypeChecker::check_type_expr_in_context(te, univ_types_in_scope, declared_typenames)
            },
            TypeExpression::TupleType(tup_types) => {
                tup_types
                .iter()
                .map(|t| TypeChecker::check_type_expr_in_context(t, univ_types_in_scope, declared_typenames))
                .fold(Ok(()), |acc, el| acc.and(el))
            },
            TypeExpression::UniversalType(tv) => {
                if univ_types_in_scope.contains(tv) {
                    Ok(())
                } else {
                    Err(format!("Type variable {} not in scope", tv))
                }
            },
            TypeExpression::FunctionType(te_func, te_arg) => {
                TypeChecker::check_type_expr_in_context(te_func, univ_types_in_scope, declared_typenames)
                .and(TypeChecker::check_type_expr_in_context(te_arg, univ_types_in_scope, declared_typenames))
            },
            // int, float, string, inferable type (because we don't yet know what to do with it) are all ok
            _ => Ok(())
        }
    }
}

// Type inference impl
impl<'a> TypeChecker<'a> {
    
}

// Type checking impl
impl<'a> TypeChecker<'a> {
    pub fn type_check_program(&self) -> Result<(), String> {
        for stmt in self.program {
            match stmt {
                Statement::UntypedLet(ids, expr) =>
                    self.type_check_untyped_let(ids, expr),
                Statement::TypedLet(id, typ, args, expr) => {
                    Ok(())
                },
                Statement::TypeDeclaration(type_id, univ_types, variants) => {
                    Ok(())
                }
            }?
        }
        Ok(())
    }

    fn type_check_untyped_let(&self, id: &Vec<String>, expr: &Expression) -> Result<(), String> {
        // need to generate a new type variable
        // also we have a function type for every identifier we have as an argument
        let initial_type = TypeExpression::UniversalType(String::from("a"));
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
