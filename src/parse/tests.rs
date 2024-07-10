use super::*;
use ordered_float::NotNan;

#[test]
fn test_parse_literal() {
    let parser = grammar::ExpressionParser::new();
    // Ok IntLiteral
    assert!(parser.parse("4").is_ok());
    assert!(parser.parse("52").is_ok());
    assert!(parser.parse("-1787234").is_ok());
    match parser.parse("675") {
        Ok(ast::Expression::IntegerLiteral(675)) => {},
        _ => panic!("675 did not parse to int 675")
    }
    // Err IntLiteral
    assert!(parser.parse("0527").is_err());
    assert!(parser.parse("-000343").is_err());
    // Ok FloatLiteral
    assert!(parser.parse("5.0").is_ok());
    assert!(parser.parse("1.0e-9").is_ok());
    assert!(parser.parse("0.23124").is_ok());
    assert!(parser.parse("1.2222E100").is_ok());
    // Err FloatLiteral
    assert!(parser.parse("00.9").is_err());
    assert!(parser.parse("4.").is_err());
    assert!(parser.parse("0.5689eE2").is_err());
    assert!(parser.parse("12.888e").is_err());
    assert!(parser.parse("3.145r10").is_err());
    assert!(parser.parse("1.2.3.4").is_err());
    // TODO: StringLiteral
}

#[test]
fn test_parse_list() {
    let parser = grammar::ExpressionParser::new();
    // Ok list literal (note: parser does no typechecking)
    match parser.parse("[]") {
        Ok(ast::Expression::List(v)) => {
            assert!(v.len() == 0);
        },
        _ => panic!("[] did not parse")
    }
    match parser.parse("[4, 5]") {
        Ok(ast::Expression::List(v)) => {
            assert!(v[0] == ast::Expression::IntegerLiteral(4));
            assert!(v[1] == ast::Expression::IntegerLiteral(5));
            assert!(v.len() == 2);
        }
        _ => panic!("[4, 5] did not parse")
    }
    match parser.parse("[1.25e-1]") {
        Ok(ast::Expression::List(v)) => {
            assert!(v.len() == 1);
            assert!(v[0] == ast::Expression::FloatLiteral(NotNan::new(0.125).unwrap()));
        }
        _ => panic!("[1.25e-1] did not parse")
    }
}