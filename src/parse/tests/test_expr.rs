use super::*;

// Expressions
#[test]
fn test_parse_literal() {
    let parser = grammar::ExpressionParser::new();

    // Ok IntLiteral
    assert!(parser.parse(false, "nothing").unwrap() == ast::Expression::Nothing(None));
    assert!(parser.parse(false, "4").unwrap() == ast::Expression::IntLiteral(4, None));
    assert!(parser.parse(false, "52").unwrap() == ast::Expression::IntLiteral(52, None));
    assert!(parser.parse(false, "-1787234").unwrap() == ast::Expression::IntLiteral(-1787234, None));
    assert!(parser.parse(false, "675").unwrap() == ast::Expression::IntLiteral(675, None));
    // Err IntLiteral
    assert!(parser.parse(false, "0527").is_err());
    assert!(parser.parse(false, "-000343").is_err());
    // Ok FloatLiteral
    assert!(parser.parse(false, "5.0").unwrap() == ast::Expression::FloatLiteral(util::to_of64(5.0), None));
    assert!(parser.parse(false, "1.0e-9").unwrap() == ast::Expression::FloatLiteral(util::to_of64(1e-9), None));
    assert!(
        parser.parse(false, "0.23124").unwrap() == ast::Expression::FloatLiteral(util::to_of64(0.23124), None)
    );
    assert!(
        parser.parse(false, "1.2222E100").unwrap()
            == ast::Expression::FloatLiteral(util::to_of64(1.2222E100), None)
    );
    // Err FloatLiteral
    assert!(parser.parse(false, "00.9").is_err());
    assert!(parser.parse(false, "4.").is_err());
    assert!(parser.parse(false, "0.5689eE2").is_err());
    assert!(parser.parse(false, "12.888e").is_err());
    assert!(parser.parse(false, "3.145r10").is_err());
    assert!(parser.parse(false, "1.2.3.4").is_err());
    assert!(parser.parse(false, "5 .0").is_err());
    // Ok StringLiteral
    assert!(
        parser.parse(false, "\"hello there\"").unwrap()
            == ast::Expression::StringLiteral(String::from("hello there"), None)
    );
    assert!(
        parser.parse(false, "\"µß£££ç∑ 😎\"").unwrap()
            == ast::Expression::StringLiteral(String::from("µß£££ç∑ 😎"), None)
    );
    assert!(parser.parse(false, "\"\"").unwrap() == ast::Expression::StringLiteral(String::from(""), None));
    // Err StringLiteral
    assert!(parser.parse(false, "\"hi there\"\"").is_err());
    assert!(parser.parse(false, "\"bruh").is_err());
    assert!(parser.parse(false, "no begin! \"").is_err());
}


#[test]
fn test_parse_list() {
    let parser = grammar::ExpressionParser::new();

    assert!(parser.parse(false, "[]").unwrap() == ast::Expression::List(vec![], None));
    assert!(
        parser.parse(false, "[-1.0e6]").unwrap()
            == ast::Expression::List(
                vec![
                    ast::Expression::FloatLiteral(util::to_of64(-1.0e6), None)
                ],
                None
            )
    );
    assert!(
        parser.parse(false, "[4, 5]").unwrap()
            == ast::Expression::List(vec![
                ast::Expression::IntLiteral(4, None),
                ast::Expression::IntLiteral(5, None),
            ], None)
    );
    assert!(
        parser
            .parse(false, "[\"buh\",4,5,7.0     , \t 8, \"⏰\"]")
            .unwrap()
            == ast::Expression::List(vec![
                ast::Expression::StringLiteral(String::from("buh"), None),
                ast::Expression::IntLiteral(4, None),
                ast::Expression::IntLiteral(5, None),
                ast::Expression::FloatLiteral(util::to_of64(7.0), None),
                ast::Expression::IntLiteral(8, None),
                ast::Expression::StringLiteral(String::from("⏰"), None)
            ], None)
    );
    // parser doesn't do type checking
    assert!(
        parser
            .parse(false, r#"[1, "wow ಣ", 1.0, (2), [46, 47, -9.85], (-52, )]"#)
            .unwrap()
            == ast::Expression::List(vec![
                ast::Expression::IntLiteral(1, None),
                ast::Expression::StringLiteral(String::from("wow ಣ"), None),
                ast::Expression::FloatLiteral(util::to_of64(1.0), None),
                ast::Expression::IntLiteral(2, None),
                ast::Expression::List(vec![
                    ast::Expression::IntLiteral(46, None),
                    ast::Expression::IntLiteral(47, None),
                    ast::Expression::FloatLiteral(util::to_of64(-9.85), None),
                ], None),
                ast::Expression::Tuple(vec![ast::Expression::IntLiteral(-52, None)], None)
            ], None)
    );
    assert!(
        parser.parse(false, "[x, 4]").unwrap()
            == ast::Expression::List(vec![
                ast::Expression::Identifier(String::from("x"), None),
                ast::Expression::IntLiteral(4, None)
            ], None)
    );

    assert!(parser.parse(false, "[,]").is_err());
    assert!(parser.parse(false, "[,7]").is_err());
    assert!(parser.parse(false, "[7,]").is_err());
    assert!(parser.parse(false, "[4, 5,]").is_err());
    assert!(parser.parse(false, "[4, -6").is_err());
    assert!(parser.parse(false, "x, 7.0, ]").is_err());
    assert!(parser.parse(false, "[").is_err());
    assert!(parser.parse(false, "]").is_err());
}


#[test]
fn test_parse_tuple() {
    let parser = grammar::ExpressionParser::new();

    assert!(parser.parse(false, "(-4)").unwrap() == ast::Expression::IntLiteral(-4, None));
    assert!(
        parser.parse(false, "(-4,)").unwrap()
            == ast::Expression::Tuple(vec![ast::Expression::IntLiteral(-4, None)], None)
    );
    assert!(
        parser.parse(false, "(5, 6, )").unwrap()
            == ast::Expression::Tuple(vec![
                ast::Expression::IntLiteral(5, None),
                ast::Expression::IntLiteral(6, None)
            ], None)
    );
    assert!(
        parser.parse(false, "(3, -7.25)").unwrap()
            == ast::Expression::Tuple(vec![
                ast::Expression::IntLiteral(3, None),
                ast::Expression::FloatLiteral(util::to_of64(-7.25), None)
            ], None)
    );

    assert!(parser.parse(false, "(").is_err());
    assert!(parser.parse(false, ")").is_err());
    assert!(parser.parse(false, "(4, 6, \"yah!\"").is_err());
    assert!(parser.parse(false, "5, 6, 3)").is_err());
}
