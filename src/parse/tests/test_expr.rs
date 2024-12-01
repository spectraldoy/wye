use super::*;
use ordered_float::OrderedFloat;

// Expressions
#[test]
fn test_parse_literal() {
    let parser = grammar::ExpressionParser::new();

    // Ok IntLiteral
    assert!(parser.parse("4").unwrap().value == ast::Expression::IntLiteral(4));
    assert!(parser.parse("52").unwrap().value == ast::Expression::IntLiteral(52));
    assert!(parser.parse("-1787234").unwrap().value == ast::Expression::IntLiteral(-1787234));
    assert!(parser.parse("675").unwrap().value == ast::Expression::IntLiteral(675));
    // Err IntLiteral
    assert!(parser.parse("0527").is_err());
    assert!(parser.parse("-000343").is_err());
    // Ok FloatLiteral
    assert!(parser.parse("5.0").unwrap().value == ast::Expression::FloatLiteral(OrderedFloat(5.0)));
    assert!(parser.parse("1.0e-9").unwrap().value == ast::Expression::FloatLiteral(OrderedFloat(1e-9)));
    assert!(
        parser.parse("0.23124").unwrap().value == ast::Expression::FloatLiteral(OrderedFloat(0.23124))
    );
    assert!(
        parser.parse("1.2222E100").unwrap().value
            == ast::Expression::FloatLiteral(OrderedFloat(1.2222E100))
    );
    // Err FloatLiteral
    assert!(parser.parse("00.9").is_err());
    assert!(parser.parse("4.").is_err());
    assert!(parser.parse("0.5689eE2").is_err());
    assert!(parser.parse("12.888e").is_err());
    assert!(parser.parse("3.145r10").is_err());
    assert!(parser.parse("1.2.3.4").is_err());
    assert!(parser.parse("5 .0").is_err());
    // Ok StringLiteral
    assert!(
        parser.parse("\"hello there\"").unwrap().value
            == ast::Expression::StringLiteral(String::from("hello there"))
    );
    assert!(
        parser.parse("\"µß£££ç∑ 😎\"").unwrap().value
            == ast::Expression::StringLiteral(String::from("µß£££ç∑ 😎"))
    );
    assert!(parser.parse("\"\"").unwrap().value == ast::Expression::StringLiteral(String::from("")));
    // Err StringLiteral
    assert!(parser.parse("\"hi there\"\"").is_err());
    assert!(parser.parse("\"bruh").is_err());
    assert!(parser.parse("no begin! \"").is_err());
}
