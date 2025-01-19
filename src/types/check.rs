//! Type checking
use super::infer;
use super::{collect_functype, Type};
use crate::parse::ast;
use crate::parse::ast::{BinaryOp, Expression, Program, Statement};
use crate::parse::span;
use crate::parse::span::GetSpan;
use std::collections::HashMap;

// TODO: are constraints about type schemes? Perhaps that is
// Something that will only be dealt with once we have working
// type inference with polymorphic types?
// Note: Wye polymorphic types assert true, total polymorphism
// and cannot be specialized at the top level, unlike type variables.

// TODO: Bounds can conflict. We need a way to resolve bounds.
pub(super) struct TypeContext {
    /// Next unused number for generating a new type variable or bound name.
    next_available_num: usize,
    /// name -> type mapping for variables declared in the program
    pub typings: HashMap<String, Type>,
    /// var name and name of bound
    quantified_typevars: HashMap<String, Option<String>>,
    /// Collect errors here to be all reported together after type checking
    /// This can be a vec of String by the way
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

    /// Advances the num next available for generating names.
    pub fn genvar(&mut self) -> usize {
        let next_num = self.next_available_num;
        self.next_available_num += 1;
        next_num
    }

    /// Creates a new String name for a bound or polytype
    pub fn genname(&mut self, prefix: String) -> String {
        format!("{}{}", prefix, self.genvar())
    }

    /// Apply a set of type constraints to the names currently declared
    /// in the type context
    pub fn ingest_subst(&mut self, subst: &HashMap<usize, Type>) {
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

    // TODO: at the end of this, all remaining unconstrained type variables
    // need to be generalized to polytypes.

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
/// This function and every mutually recursive function it calls is  responsible
/// for applying the resulting substitution to the resulting type, and the context.
pub(super) fn type_check_expr(
    expr: &Expression,
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<usize, Type>), ()> {
    match expr {
        Expression::Nothing(_) => Ok((Type::None, HashMap::new())),
        Expression::IntLiteral(_, _) => Ok((Type::Int, HashMap::new())),
        Expression::FloatLiteral(_, _) => Ok((Type::Float, HashMap::new())),
        Expression::StringLiteral(_, _) => Ok((Type::String, HashMap::new())),
        Expression::List(exprs, _) => type_check_list(&exprs[..], ctx),
        Expression::BinaryOp(bop, _) => type_check_binary_op(bop, ctx),
        Expression::FuncApplication(func, args, _) => {
            let (func_type, func_subst) = type_check_expr(func, ctx)?;
            type_check_func_app(func_type, func_subst, func.get_span(), args, ctx)
        }
        Expression::Let(varwithval, in_expr_opt, span) => {
            if let Some(in_expr) = in_expr_opt {
                type_check_let_in(varwithval, in_expr, span.as_ref().unwrap().clone(), ctx)
            } else {
                type_check_let(varwithval, span.as_ref().unwrap().clone(), ctx)
            }
        }
        _ => todo!(),
    }
}

// TODO: rename substitution to constraint
/// Type check a slice of expressions that are meant to be the contents
/// of a List expression
fn type_check_list(
    exprs: &[Expression],
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<usize, Type>), ()> {
    if exprs.len() == 0 {
        // Empty lists always type to [t] where t is a new type variable.
        let new_typevar = ctx.genvar();
        // TODO: this should be a true polytype - a forall
        // When it is referenced from the environment, it should be instantiated to
        // a new type variable
        // This would happen in let statements where these type variables
        // Are elevated to polytypes with the name pi{x} or something
        return Ok((
            Type::List(Box::new(Type::Variable(new_typevar))),
            HashMap::new(),
        ));
    }

    // Type check each expression
    let (elem_types, mut composed_subst) = type_check_nonempty_expr_slice(exprs, ctx)?;

    // Now unify all of the elem types
    let mut cur_unified_type = elem_types[0].clone();
    for (i, typ) in elem_types.iter().enumerate().skip(1) {
        let mut unif_subst = HashMap::new();
        let unif_res = infer::unify(&cur_unified_type, typ, &mut unif_subst);
        if unif_res.is_err() {
            let error_span =
                span::widest_span(&[exprs[i - 1].get_span(), exprs[i].get_span()]).unwrap();
            ctx.type_errors.insert(
                error_span,
                format!(
                    "Could not unify types of elements in list. Got {:?} which is not compatible with {:?}: {}",
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
        ctx.ingest_subst(&composed_subst);
    }

    Ok((Type::List(Box::new(cur_unified_type)), composed_subst))
}

/// Produce the type of builtin binary operations
/// BIG TODO: type variables need bounds
fn type_check_binary_op(
    bop: &BinaryOp,
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<usize, Type>), ()> {
    let new_type = match bop {
        BinaryOp::Add => Type::Function(
            Box::new(Type::Int),
            Box::new(Type::Function(Box::new(Type::Int), Box::new(Type::Int))),
        ),
        _ => todo!(),
    };
    Ok((new_type, HashMap::new()))
}

/// Type check a function application. This takes in the type and substitutions obtained from type
/// checking the actual function so that recursion is easier.
fn type_check_func_app(
    func_type: Type,
    input_subst: HashMap<usize, Type>,
    func_span: span::Span,
    args: &[Expression],
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<usize, Type>), ()> {
    // If the function has no arguments, then its type is not changed in any way, as it
    // is not applied.
    if args.len() == 0 {
        return Ok((func_type, input_subst));
    }

    // Expect the function expression to have Function type
    let (expected_arg_type, ret_type) = if let Type::Function(arg_type, ret_type) = func_type {
        (arg_type, ret_type)
    } else {
        ctx.type_errors.insert(
            func_span,
            format!("Expected function type but got {:?}", func_type),
        );
        return Err(());
    };

    // Type check the first arg
    let (first_arg_type, first_arg_subst) = type_check_expr(&args[0], ctx)?;
    let mut composed_subst = infer::compose_substs(&first_arg_subst, &input_subst);

    // From the function
    let expected_arg_type = infer::apply_subst_type(&composed_subst, &expected_arg_type);

    // Unify the expected argument type of the function and the actual type of the first argument
    let mut unif_subst = HashMap::new();
    let unif_res = infer::unify(&expected_arg_type, &first_arg_type, &mut unif_subst);
    let func_and_first_arg_span = span::widest_span(&[func_span, args[0].get_span()]).unwrap();
    if unif_res.is_err() {
        ctx.type_errors.insert(
            func_and_first_arg_span,
            format!(
                "Could not unify expected argument type {:?} with actual {:?}",
                expected_arg_type, first_arg_type,
            ),
        );
        return Err(());
    }
    // Incorporate the unification substitution into the current substitution
    composed_subst = infer::compose_substs(&unif_subst, &composed_subst);
    // Apply this substitution to the relevant types
    let ret_type = infer::apply_subst_type(&composed_subst, &ret_type);

    // Recurse to type the rest of the application.
    type_check_func_app(
        ret_type,
        composed_subst,
        func_and_first_arg_span,
        &args[1..],
        ctx,
    )
}

// TODO: rename TypeContext to TypeChecker and have all these functions in
// the impl of it
// TODO: move this function around
/// Returns a vec of types of the expressions in the slice, and the substitution
/// composed by all of the substitutions induced by typing the expressions
fn type_check_nonempty_expr_slice(
    exprs: &[Expression],
    ctx: &mut TypeContext,
) -> Result<(Vec<Type>, HashMap<usize, Type>), ()> {
    let mut elem_types = vec![];
    let mut composed_subst = HashMap::new();
    for expr in exprs {
        // Recursively type check each expression
        // need to unify no
        let (elem_type, elem_subst) = type_check_expr(expr, ctx)?;
        // Compose the obtained substitutions together
        composed_subst = infer::compose_substs(&elem_subst, &composed_subst);
        // Apply to the output type and the context
        elem_types.push(infer::apply_subst_type(&composed_subst, &elem_type));
        ctx.ingest_subst(&composed_subst);
    }
    Ok((elem_types, composed_subst))
}

/// Type check a let that does not have an in expression.
fn type_check_let(
    varwithval: &ast::VarWithValue,
    span: span::Span,
    ctx: &mut TypeContext,
) -> Result<(Type, HashMap<usize, Type>), ()> {
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
        let func_type_res = collect_functype(&arg_types[..]);
        if func_type_res.is_err() {
            ctx.type_errors
                .insert(span, func_type_res.err().unwrap().to_string());
            return Err(());
        }
        func_type_res.unwrap()
    };
    ctx.typings.insert(name.clone(), func_type);

    // Type check the expression, apply the obtained substitutions to the environment
    // and to the type of the expression
    let (expr_type, expr_subst) = type_check_expr(expr, ctx)?;
    ctx.ingest_subst(&expr_subst);
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
    ctx.ingest_subst(&final_subst);
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
) -> Result<(Type, HashMap<usize, Type>), ()> {
    todo!()
}
