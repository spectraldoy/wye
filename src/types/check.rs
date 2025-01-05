use super::infer;
/// Type checking
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

struct TypeContext {
    next_available_num: u128,
    // name -> type mapping for variables declared in the program
    pub typings: HashMap<String, Type>,
    // var name and name of bound
    quantified_typevars: HashMap<String, Option<String>>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            next_available_num: 0,
            typings: HashMap::new(),
            quantified_typevars: HashMap::new(),
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

pub fn type_check_program(prog: &Program) -> Result<(), HashMap<String, span::Span>> {
    let mut ctx = TypeContext::new();

    for stmt in prog {
        type_check_statement(stmt, &mut ctx)?;
    }

    Ok(())
}

fn type_check_statement(
    stmt: &Statement,
    ctx: &mut TypeContext,
) -> Result<(), HashMap<String, span::Span>> {
    match stmt {
        Statement::Expression(expr) => {
            let res = type_check_expr(expr, ctx);
            if res.is_err() {
                return Err(res.err().unwrap());
            }
        }
        _ => todo!(),
    }

    Ok(())
}

/// Alias to avoid updating this huge type in multiple places.
/// TODO: we may not need a hashmap of errors
type ExprCheck = Result<(Type, HashMap<u128, Type>), HashMap<String, span::Span>>;

/// Return the inferred sub-expr type, and resulting substitutions for inference
fn type_check_expr(expr: &Expression, ctx: &mut TypeContext) -> ExprCheck {
    let recur_res = match expr {
        Expression::Nothing(_) => Ok((Type::None, HashMap::new())),
        Expression::IntLiteral(_, _) => Ok((Type::Int, HashMap::new())),
        Expression::FloatLiteral(_, _) => Ok((Type::Float, HashMap::new())),
        Expression::List(exprs, span) => {
            type_check_list(&exprs[..], span.as_ref().unwrap().clone(), ctx)
        }
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
    return Ok((typ, subst));
}

// This seems much more complicated than it needs to be. Can it be unraveled?
// It's possible there are too many ctx.apply_subst s
fn type_check_list(exprs: &[Expression], span: span::Span, ctx: &mut TypeContext) -> ExprCheck {
    if exprs.len() == 0 {
        // Empty lists always type to [t] where t is a new type variable.
        let new_typevar = ctx.genvar();
        return Ok((
            Type::List(Box::new(Type::Variable(new_typevar))),
            HashMap::new(),
        ));
    }

    // Distinguish the head and the tail for easy recursion.
    let head = &exprs[0];
    let tail = &exprs[1..];

    // Compute the span for the tail, for error reporting.
    let tail_span = if let Some(s) = span::widest_span(&span::get_seq_spans(tail)) {
        s
    } else {
        span.clone()
    };

    // Recursively type check the head
    let (head_type, head_subst) = type_check_expr(head, ctx)?;
    ctx.apply_subst(&head_subst);
    // Recursively type check the tail
    let (tail_type, tail_subst) = type_check_list(tail, tail_span.clone(), ctx)?;
    ctx.apply_subst(&tail_subst);

    // Compose the substitution-sets obtained from each
    let composed_subst = infer::compose_substs(&head_subst, &tail_subst);

    // Apply the composed substitutions to the types we wish to unify
    let head_type = infer::apply_subst_type(&composed_subst, &head_type);
    let tail_type = infer::apply_subst_type(&composed_subst, &tail_type);

    // Expect the tail to be a list type, then try to unify with the head type
    if let Type::List(t) = tail_type {
        let mut unif_subst = HashMap::new();
        let unif_res = infer::unify(&head_type, &t, &mut unif_subst);
        if unif_res.is_err() {
            // report the error back with spans
            return Err(HashMap::from([(unif_res.err().unwrap(), span)]));
        }
        let final_subst = infer::compose_substs(&composed_subst, &unif_subst);
        ctx.apply_subst(&final_subst);
        let final_type = infer::apply_subst_type(&final_subst, &head_type);
        Ok((final_type, final_subst))
    } else {
        Err(HashMap::from([(
            format!("Expected a list type, but got {:?}", tail_type),
            tail_span,
        )]))
    }
}

/// Type check a let that does not have an in expression.
fn type_check_let(
    varwithval: &ast::VarWithValue,
    span: span::Span,
    ctx: &mut TypeContext,
) -> ExprCheck {
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
    let func_type = Type::Function(arg_types);
    ctx.typings.insert(name.clone(), func_type.clone());

    // Type check the expression
    let (expr_type, expr_subst) = type_check_expr(expr, ctx)?;
    ctx.apply_subst(&expr_subst);

    // Unify expr_type and output_type
    let mut unif_subst = HashMap::new();
    let unif_res = infer::unify(&expr_type, &output_type, &mut unif_subst);
    if unif_res.is_err() {
        // report the error back with spans
        return Err(HashMap::from([(unif_res.err().unwrap(), span)]));
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
) -> ExprCheck {
    todo!()
}
