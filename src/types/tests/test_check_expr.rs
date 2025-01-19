use super::Type::*;
use super::*;
use crate::parse::ast::{BinaryOp, Expression};
use crate::test_util::to_of64;
// override the imported Type::None
use crate::parse::span::{GetSpan, Span};
use check::type_check_expr;
use Option::None;

#[cfg(test)]
impl GetSpan for Expression {
    fn get_span(&self) -> Span {
        return Span::new(0, 0);
    }
}

// Assume validly formed expression
fn test_check_expr(expr: Expression) -> Result<Type, ()> {
    let mut ctx = check::TypeContext::new();
    let res = check::type_check_expr(&expr, &mut ctx);
    return res.map(|(typ, _)| typ);
}

#[test]
fn test_check_literal() {
    assert_eq!(
        test_check_expr(Expression::IntLiteral(4, None)).unwrap(),
        Int
    );
    assert_eq!(
        test_check_expr(Expression::StringLiteral("hello".to_string(), None)).unwrap(),
        String
    );
    assert_eq!(
        test_check_expr(Expression::Nothing(None)).unwrap(),
        Type::None
    );
    assert_eq!(
        test_check_expr(Expression::FloatLiteral(to_of64(4.5), None)).unwrap(),
        Float
    );
}

#[test]
fn test_check_list() {
    // Simple lists
    assert_eq!(
        test_check_expr(Expression::List(
            vec![
                Expression::IntLiteral(4, None),
                Expression::IntLiteral(5, None),
            ],
            None
        ))
        .unwrap(),
        List(Box::new(Int))
    );
    assert_eq!(
        test_check_expr(Expression::List(
            vec![Expression::IntLiteral(4, None),],
            None
        ))
        .unwrap(),
        List(Box::new(Int))
    );
    assert_eq!(
        test_check_expr(Expression::List(vec![], None)).unwrap(),
        List(Box::new(Variable(0)))
    );

    assert!(test_check_expr(Expression::List(
        vec![
            Expression::IntLiteral(4, None),
            Expression::FloatLiteral(to_of64(5.6), None),
        ],
        None
    ))
    .is_err());
}

#[test]
fn test_check_func_application() {
    assert_eq!(
        test_check_expr(Expression::FuncApplication(
            Box::new(Expression::BinaryOp(BinaryOp::Add, None)),
            vec![
                Expression::IntLiteral(4, None),
                Expression::IntLiteral(5, None),
            ],
            None,
        ))
        .unwrap(),
        Int
    );
}
