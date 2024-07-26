/// Utility functions to help with type checking
use crate::parse::ast::TypeExpression;

pub fn inftype_name(k: usize) -> String {
    return format!("τ{}", k);
}

/// Collects a slice of many type expressions, in order, into a single function type
/// In the case that there is only one type present in the input slice, it just
/// returns that type - no wrapping inside any kind of function type
pub fn collect_functype(types: &[TypeExpression]) -> TypeExpression {
    if types.len() == 1 {
        return types[0].clone();
    }

    let rest_type = collect_functype(&types[1..]);
    TypeExpression::FunctionType(
        Box::new(types[0].clone()),
        Box::new(rest_type)
    )
}

/// Substitute an inferable type with for_type in type expression in_type
/// We expect by this time that all type expressions in the program have been checked
/// to use the correct number of type arguments for declared types and to only use
/// type variables that are present in the scope of the type
pub fn subst_inftype_in_type_expr(inftype: &String, for_type: &TypeExpression, in_type: &TypeExpression) -> TypeExpression {
    match in_type {
        TypeExpression::DeclaredType(typename, type_args) => {
            let mapped_args: Vec<TypeExpression> = type_args
                .iter()
                .map(|ta| subst_inftype_in_type_expr(inftype, for_type, ta))
                .collect::<_>();
            TypeExpression::DeclaredType(typename.clone(), mapped_args)
        },
        TypeExpression::ListType(te) => {
            TypeExpression::ListType(Box::new(subst_inftype_in_type_expr(inftype, for_type, te)))
        },
        TypeExpression::TupleType(tup_types) => {
            let mapped_types: Vec<TypeExpression> = tup_types
                .iter()
                .map(|t| subst_inftype_in_type_expr(inftype, for_type, t))
                .collect::<_>();
            TypeExpression::TupleType(mapped_types)
        },
        TypeExpression::FunctionType(te_func, te_arg) => {
            TypeExpression::FunctionType(
                Box::new(subst_inftype_in_type_expr(inftype, for_type, te_func)),
                Box::new(subst_inftype_in_type_expr(inftype, for_type, te_arg))
            )
        },
        // here we actually perform the substitution
        TypeExpression::InferableType(_) => for_type.clone(),
        // Any of the other types require no substitution
        _ => in_type.clone()
    }
}
