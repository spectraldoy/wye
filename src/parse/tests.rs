use super::*;
use ordered_float::OrderedFloat;

#[test]
fn test_parse_literal() {
    let parser = grammar::ExpressionParser::new();
    // Ok IntLiteral
    assert!(parser.parse("4").unwrap() == ast::Expression::IntegerLiteral(4));
    assert!(parser.parse("52").unwrap() == ast::Expression::IntegerLiteral(52));
    assert!(parser.parse("-1787234").unwrap() == ast::Expression::IntegerLiteral(-1787234));
    assert!(parser.parse("675").unwrap() == ast::Expression::IntegerLiteral(675));
    // Err IntLiteral
    assert!(parser.parse("0527").is_err());
    assert!(parser.parse("-000343").is_err());
    // Ok FloatLiteral
    assert!(parser.parse("5.0").unwrap() == ast::Expression::FloatLiteral(OrderedFloat(5.0)));
    assert!(parser.parse("1.0e-9").unwrap() == ast::Expression::FloatLiteral(OrderedFloat(1e-9)));
    assert!(parser.parse("0.23124").unwrap() == ast::Expression::FloatLiteral(OrderedFloat(0.23124)));
    assert!(parser.parse("1.2222E100").unwrap() == ast::Expression::FloatLiteral(OrderedFloat(1.2222E100)));
    // Err FloatLiteral
    assert!(parser.parse("00.9").is_err());
    assert!(parser.parse("4.").is_err());
    assert!(parser.parse("0.5689eE2").is_err());
    assert!(parser.parse("12.888e").is_err());
    assert!(parser.parse("3.145r10").is_err());
    assert!(parser.parse("1.2.3.4").is_err());
    // Ok StringLiteral
    assert!(parser.parse("\"hello there\"").unwrap() == ast::Expression::StringLiteral(String::from("hello there")));
    assert!(parser.parse("\"µß£££ç∑ 😎\"").unwrap() == ast::Expression::StringLiteral(String::from("µß£££ç∑ 😎")));
    // Err StringLiteral
    assert!(parser.parse("\"hi there\"\"").is_err());
}


#[test]
fn test_parse_list() {
    let parser = grammar::ExpressionParser::new();
    assert!(parser.parse("[]").unwrap() == ast::Expression::List(vec![]));
    assert!(parser.parse("[4, 5]").unwrap() == ast::Expression::List(vec![
        ast::Expression::IntegerLiteral(4),
        ast::Expression::IntegerLiteral(5),
    ]));
    // parser doesn't do type checking
    assert!(parser.parse(r#"[1, "wow ಣ", 1.0, (2), [46, 47], (-52, )]"#).unwrap() == ast::Expression::List(vec![
        ast::Expression::IntegerLiteral(1),
        ast::Expression::StringLiteral(String::from("wow ಣ")),
        ast::Expression::FloatLiteral(OrderedFloat(1.0)),
        ast::Expression::IntegerLiteral(2),
        ast::Expression::List(vec![
            ast::Expression::IntegerLiteral(46),
            ast::Expression::IntegerLiteral(47),
        ]),
        ast::Expression::Tuple(vec![ast::Expression::IntegerLiteral(-52)])
    ]));

    // List of lists

    // Err
    assert!(parser.parse("[4, 5,]").is_err());
}

// TODO: tuple tests

// #[test]
// fn test_parse_typevariant() {
//     let parser = grammar::ExpressionParser::new();
    
// }

