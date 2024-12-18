use super::*;
use super::span::Span;
use util;
use lalrpop_util::ParseError;

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
            == ast::Expression::StringLiteral("hello there".to_string(), None)
    );
    assert!(
        parser.parse(false, "\"µß£££ç∑ 😎\"").unwrap()
            == ast::Expression::StringLiteral("µß£££ç∑ 😎".to_string(), None)
    );
    assert!(parser.parse(false, "\"\"").unwrap() == ast::Expression::StringLiteral("".to_string(), None));
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
                ast::Expression::StringLiteral("buh".to_string(), None),
                ast::Expression::IntLiteral(4, None),
                ast::Expression::IntLiteral(5, None),
                ast::Expression::FloatLiteral(util::to_of64(7.0), None),
                ast::Expression::IntLiteral(8, None),
                ast::Expression::StringLiteral("⏰".to_string(), None)
            ], None)
    );
    // parser doesn't do type checking
    assert!(
        parser
            .parse(false, r#"[1, "wow ಣ", 1.0, (2), [46, 47, -9.85], (-52, )]"#)
            .unwrap()
            == ast::Expression::List(vec![
                ast::Expression::IntLiteral(1, None),
                ast::Expression::StringLiteral("wow ಣ".to_string(), None),
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
                ast::Expression::Identifier("x".to_string(), None),
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

#[test]
fn test_parse_record_expr() {
    let parser = grammar::ExpressionParser::new();

    assert!(parser.parse(false, "{field: Field.feeld}").unwrap()
        == ast::Expression::Record(vec![
            ("field".to_string(), ast::Expression::Projection("Field".to_string(), "feeld".to_string(), None), None)
        ],
        None)
    );
    assert!(parser.parse(false, "{
        bint: 3,
        jint: [2],
        cidnt: (1),
        lint: (\"hi\",)
    }").unwrap() == ast::Expression::Record(vec![
        ("bint".to_string(), ast::Expression::IntLiteral(3, None), None),
        ("jint".to_string(), ast::Expression::List(vec![ast::Expression::IntLiteral(2, None)], None), None),
        ("cidnt".to_string(), ast::Expression::IntLiteral(1, None), None),
        ("lint".to_string(), ast::Expression::Tuple(vec![ast::Expression::StringLiteral("hi".to_string(), None)], None), None)
    ], None));
    assert!(parser.parse(false, "({one: 2, three: 4})").unwrap()
        == ast::Expression::Record(vec![
            ("one".to_string(), ast::Expression::IntLiteral(2, None), None),
            ("three".to_string(), ast::Expression::IntLiteral(4, None), None)
        ], None));
    
    assert!(parser.parse(false, "({one:2,three:4},)").unwrap()
        == ast::Expression::Tuple(vec![ast::Expression::Record(vec![
            ("one".to_string(), ast::Expression::IntLiteral(2, None), None),
            ("three".to_string(), ast::Expression::IntLiteral(4, None), None)
        ], None)], None));

    assert!(parser.parse(false, "{}").is_err());
    assert!(parser.parse(false, "{super(pub): 4}").is_err());
    assert!(parser.parse(false, "{super: 4,}").is_err());
    assert!(parser.parse(false, "{int: 4}").is_err());
    assert!(parser.parse(false, "{4: thing}").is_err());
    assert!(parser.parse(false, "unclosed: curly}").is_err());
    assert!(parser.parse(false, "{one: two three: four}").is_err());
}

#[test]
fn test_parse_identifier() {
    let parser = grammar::ExpressionParser::new();

    assert!(parser.parse(false, "x").unwrap() == ast::Expression::Identifier("x".to_string(), None));
    assert!(
        parser.parse(false, "identif").unwrap() == ast::Expression::Identifier("identif".to_string(), None)
    );
    assert!(parser.parse(false, "hElO_").unwrap() == ast::Expression::Identifier("hElO_".to_string(), None));
    assert!(parser.parse(false, "_a0001").unwrap() == ast::Expression::Identifier("_a0001".to_string(), None));
    assert!(parser.parse(false, "Hello").unwrap() == ast::Expression::Identifier("Hello".to_string(), None));
    assert!(
        parser.parse(false, "__Option").unwrap() == ast::Expression::Identifier("__Option".to_string(), None)
    );
    assert!(
        parser.parse(false, "Ty6_Var68__iant_").unwrap()
            == ast::Expression::Identifier("Ty6_Var68__iant_".to_string(), None)
    );
    assert!(parser.parse(false, "___01").unwrap() == ast::Expression::Identifier("___01".to_string(), None));
    assert!(parser.parse(false, "___").unwrap() == ast::Expression::Identifier("___".to_string(), None));
    assert!(parser.parse(false, "(<)").unwrap() == ast::Expression::BinaryOp(ast::BinaryOp::Lt, None));
    assert!(parser.parse(false, "(+)").unwrap() == ast::Expression::BinaryOp(ast::BinaryOp::Add, None));
    assert!(parser.parse(false, "(//)").unwrap() == ast::Expression::BinaryOp(ast::BinaryOp::FloorDiv, None));

    assert!(parser.parse(false, "string").is_err());
    assert!(parser.parse(false, "with").is_err());
    assert!(parser.parse(false, "int").is_err());
    assert!(parser.parse(false, "<").is_err());
    assert!(parser.parse(false, "(-").is_err());
    assert!(parser.parse(false, "a*").is_err());
    assert!(parser.parse(false, "//)").is_err());
    assert!(parser.parse(false, "yel⏰o").is_err());
    assert!(parser.parse(false, "31232abcd").is_err());
    assert!(parser.parse(false, "Hel)lo").is_err());
    assert!(parser.parse(false, "31232_AA").is_err());
    assert!(parser.parse(false, "_Yel⏰o").is_err());
    assert!(parser.parse(false, "aபாதை").is_err());
}

#[test]
fn test_parse_enum_variant() {
    let parser = grammar::ExpressionParser::new();

    assert!(parser.parse(false, "Card.King").unwrap() == ast::Expression::Projection("Card".to_string(), "King".to_string(), None));

    assert!(parser.parse(false, "Option.Some with 4").unwrap() == ast::Expression::EnumVariant {
        enum_id: "Option".to_string(),
        variant: "Some".to_string(),
        field: Box::new(ast::Expression::IntLiteral(4, None)),
        span: None
    });

    // I'm thinking that 
    assert!(parser.parse(false, "(Thing.thing with 3)").unwrap() == ast::Expression::EnumVariant {
        enum_id: "Thing".to_string(),
        variant: "thing".to_string(),
        field: Box::new(ast::Expression::IntLiteral(3, None)),
        span: None,
    });

    assert!(parser.parse(
        false,
        "Tree.Node with (
            (Tree.Node with (Tree.Leaf, Tree.Leaf, -2.5)),
            Tree.Leaf,
            7
        )"
    ).unwrap() == ast::Expression::EnumVariant {
        enum_id: "Tree".to_string(),
        variant: "Node".to_string(),
        span: None,
        field: Box::new(
            ast::Expression::Tuple(vec![
                ast::Expression::EnumVariant {
                    enum_id: "Tree".to_string(),
                    variant: "Node".to_string(),
                    span: None,
                    field: Box::new(
                        ast::Expression::Tuple(vec![
                            ast::Expression::Projection("Tree".to_string(), "Leaf".to_string(), None),
                            ast::Expression::Projection("Tree".to_string(), "Leaf".to_string(), None),
                            ast::Expression::FloatLiteral(util::to_of64(-2.5), None)
                        ], None)
                    )
                },
                ast::Expression::Projection("Tree".to_string(), "Leaf".to_string(), None),
                ast::Expression::IntLiteral(7, None),
            ], None)
        )
    });
    assert!(parser.parse(false, "Listy.Listy with [1, \"hell⏰\"]").unwrap()
        == ast::Expression::EnumVariant {
            enum_id: "Listy".to_string(),
            variant: "Listy".to_string(),
            span: None,
            field: Box::new(ast::Expression::List(vec![
                ast::Expression::IntLiteral(1, None),
                ast::Expression::StringLiteral("hell⏰".to_string(), None),
            ], None))
        }
    );
    assert!(parser.parse(false, "Tupy.MaybeTuple with (-5.2)").unwrap()
        == ast::Expression::EnumVariant {
            enum_id: "Tupy".to_string(),
            variant: "MaybeTuple".to_string(),
            span: None,
            field: Box::new(ast::Expression::FloatLiteral(util::to_of64(-5.2), None))
        }
    );

    // Missing parenthesis
    assert!(parser.parse(
        false,
        "Tree.Node with (
            (Tree.Node with (Tree.Leaf, Tree.Leaf, -2.5),
            Tree.Leaf,
            7
        )"
    ).is_err());
    // No projection
    assert!(parser.parse(false, "Listy with [1, \"hell⏰\"])").is_err());
    // Trailing unmatched parenthesis
    assert!(parser.parse(false, "Listy.Listy with [1, \"hell⏰\"])").is_err());

    assert!(parser.parse(false, "x.y").unwrap() == ast::Expression::Projection("x".to_string(), "y".to_string(), None));
    assert!(parser.parse(false, "0xy.var").is_err());
    assert!(parser.parse(false, "xy0.__xy").unwrap() == ast::Expression::Projection("xy0".to_string(), "__xy".to_string(), None));
    assert!(parser.parse(false, "__9._a5").unwrap() == ast::Expression::Projection("__9".to_string(), "_a5".to_string(), None));
    assert!(parser.parse(false, "xs*.bruh").is_err());
    assert!(parser.parse(false, "x.8").is_err());
    assert!(parser.parse(false, "Yu.p with [8, 78").is_err());
    assert!(parser.parse(false, "Option.Some int").is_err());
    assert!(parser.parse(false, "He)i.k with 4").is_err());
    // Tokens should have at least a space between them
    // Collect spans to check collision
    assert!(parser.parse(true, "(a_9.u8)with \"hi\"").is_err());
    assert!(matches!(
        parser.parse(true, "  thingy.thing with\"hi\"").err().unwrap(),
        ParseError::User { error: (e, s) }
        if e.contains("Space required") && s == Some(Span::new(15, 23))
    ));
}
