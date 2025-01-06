/// Type checking
use super::infer;
use super::Type;
use crate::parse::ast;
use crate::parse::ast::{Expression, Program, Statement};
use crate::parse::span;
use crate::parse::span::GetSpan;
use std::collections::HashMap;

// TODO: are constraints about type schemes? Perhaps that is
// Something that will only be dealt with once we have working
// type inference with polymorphic types?
// Note: Wye polymorphic types assert true, total polymorphism
// and cannot be specialized, unlike type variables.

pub(super) struct TypeContext {
    next_available_num: u128,
    // name -> type mapping for variables declared in the program
    pub typings: HashMap<String, Type>,
    // var name and name of bound
    quantified_typevars: HashMap<String, Option<String>>,
    // Collect errors here to be all reported together after type checking
    pub type_errors: HashMap<span::Span, String>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            next_available_num: 0,
            typings: HashMap::new(),
            quantified_typevars: HashMap::new(),
            type_errors: HashMap::new(),
        }
    }

    /// Creates a new type variable of the form π{num} and
    /// adds it to the typings with
    pub fn genvar(&mut self) -> u128 {
        let next_num = self.next_available_num;
        self.next_available_num += 1;
        next_num
    }

    /// Apply a set of type constraints to the names currently declared
    /// in the type context
    pub fn apply_subst(&mut self, subst: &HashMap<u128, Type>) {
        for typ in self.typings.values_mut() {
            *typ = infer::apply_subst_type(subst, &typ);
        }
    }
}

pub fn type_check_program(prog: &Program) -> Result<(), HashMap<span::Span, String>> {
    let mut ctx = TypeContext::new();

    for stmt in prog {
        let _ = type_check_statement(stmt, &mut ctx);
    }

    if (&ctx.type_errors).is_empty() {
        Ok(())
    } else {
        Err(ctx.type_errors)
    }
}

fn type_check_statement(stmt: &Statement, ctx: &mut TypeContext) -> Result<(), ()> {
    match stmt {
        Statement::Expression(expr) => {
            // Expressions are responsible for pushing errors into the environment
            let res = type_check_expr(expr, ctx);
            if res.is_err() {
                return Err(());
            }
        }
        _ => todo!(),
    }

    Ok(())
}

/// Return the inferred sub-expr type, and resulting substitutions for inference
pub(super) fn type_check_expr(
    expr: &Expression,
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<u128, Type>), ()> {
    let recur_res = match expr {
        Expression::Nothing(_) => Ok((Type::None, HashMap::new())),
        Expression::IntLiteral(_, _) => Ok((Type::Int, HashMap::new())),
        Expression::FloatLiteral(_, _) => Ok((Type::Float, HashMap::new())),
        Expression::List(exprs, _) => type_check_list(&exprs[..], ctx),
        Expression::Let(varwithval, in_expr_opt, span) => {
            if let Some(in_expr) = in_expr_opt {
                type_check_let_in(varwithval, in_expr, span.as_ref().unwrap().clone(), ctx)
            } else {
                type_check_let(varwithval, span.as_ref().unwrap().clone(), ctx)
            }
        }
        _ => todo!(),
    };
    if recur_res.is_err() {
        return recur_res;
    }
    let (typ, subst) = recur_res.unwrap();
    ctx.apply_subst(&subst);
    let typ = infer::apply_subst_type(&subst, &typ);
    return Ok((typ, subst));
}

// This seems much more complicated than it needs to be. Can it be unraveled?
// It's possible there are too many ctx.apply_subst s
fn type_check_list(
    exprs: &[Expression],
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<u128, Type>), ()> {
    if exprs.len() == 0 {
        // Empty lists always type to [t] where t is a new type variable.
        let new_typevar = ctx.genvar();
        return Ok((
            Type::List(Box::new(Type::Variable(new_typevar))),
            HashMap::new(),
        ));
    }

    // Type check each expression
    let mut elem_types = vec![];
    let mut elem_substs = vec![];
    for expr in exprs {
        let (elem_type, elem_subst) = type_check_expr(expr, ctx)?;
        elem_types.push(elem_type);
        elem_substs.push(elem_subst);
    }

    let mut composed_subst = HashMap::new();
    for (i, subst) in elem_substs.iter().enumerate() {
        // Apply each obtained substitution to the environment
        ctx.apply_subst(subst);
        // And to the type of the corresponding recursively obtained element
        elem_types[i] = infer::apply_subst_type(subst, &elem_types[i]);
        // And compose them all together
        composed_subst = infer::compose_substs(&composed_subst, subst);
    }

    // Now unify all of the elem types
    let mut cur_unified_type = elem_types[0].clone();
    for (i, typ) in elem_types[1..].iter().enumerate() {
        let mut unif_subst = HashMap::new();
        let unif_res = infer::unify(&cur_unified_type, typ, &mut unif_subst);
        if unif_res.is_err() {
            let error_span =
                span::widest_span(&[exprs[i - 1].get_span(), exprs[i].get_span()]).unwrap();
            ctx.type_errors.insert(
                error_span,
                format!(
                    "Could not unify types of elements in list. Got {:?} which is not compatiable with {:?}: {}",
                    typ,
                    cur_unified_type,
                    unif_res.err().unwrap(),
                )
            );
            return Err(());
        }

        // Apply the substitution to the current type
        composed_subst = infer::compose_substs(&unif_subst, &composed_subst);
        cur_unified_type = infer::apply_subst_type(&composed_subst, &cur_unified_type);
        ctx.apply_subst(&composed_subst);
    }

    Ok((cur_unified_type, composed_subst))
}

/// Type check a let that does not have an in expression.
fn type_check_let(
    varwithval: &ast::VarWithValue,
    span: span::Span,
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<u128, Type>), ()> {
    let ast::VarWithValue {
        name: (name, _),
        args,
        rec: _,
        expr,
    } = varwithval;

    // Create type variables for each argument
    let mut arg_types = vec![];
    for arg in args {
        // TODO: check for duplicate argument names
        let new_type = Type::Variable(ctx.genvar());
        arg_types.push(new_type.clone());
        ctx.typings.insert(arg.0.clone(), new_type);
    }
    // Also create a type variable for the output type of the function
    let output_type = Type::Variable(ctx.genvar());
    arg_types.push(output_type.clone());

    // If recursion is allowed, then the current function should be added to
    // the type context. Suppose recursion is allowed for now
    // TODO: make recursion opt-in
    let func_type = if args.len() == 0 {
        output_type.clone()
    } else {
        Type::Function(arg_types)
    };
    ctx.typings.insert(name.clone(), func_type);

    // Type check the expression, apply the obtained substitutions to the environment
    // and to the type of the expression
    let (expr_type, expr_subst) = type_check_expr(expr, ctx)?;
    ctx.apply_subst(&expr_subst);
    let expr_type = infer::apply_subst_type(&expr_subst, &expr_type);

    // Unify expr_type and output_type
    let mut unif_subst = HashMap::new();
    let unif_res = infer::unify(&expr_type, &output_type, &mut unif_subst);
    if unif_res.is_err() {
        // report the error to the type context
        ctx.type_errors.insert(
            span,
            format!(
                "Could not unify variable type {:?} with type of expression assigned to it {:?}: {}",
                output_type,
                expr_type,
                unif_res.err().unwrap(),
            )
        );

        return Err(());
    }
    let final_subst = infer::compose_substs(&expr_subst, &unif_subst);
    ctx.apply_subst(&final_subst);
    let final_type = ctx.typings.get(name).unwrap();
    Ok((final_type.clone(), final_subst))
}

/// Type check a let expression that has an in expression
#[allow(unused_variables)]
fn type_check_let_in(
    varwithval: &ast::VarWithValue,
    in_expr: &Expression,
    span: span::Span,
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<u128, Type>), ()> {
    todo!()
}
