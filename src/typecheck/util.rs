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
