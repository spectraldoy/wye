use super::*;
use crate::parse::ast::Expression;

// Assume validly formed expression
fn test_check_expr(expr: Expression) -> Type {
    let mut ctx = check::TypeContext::new();
    let res = check::type_check_expr(&expr, &mut ctx);
    return res.ok().unwrap().0;
}

#[test]
fn test_check_literal() {
    assert!(test_check_expr(Expression::IntLiteral(4, None)) == Type::Int);
}
