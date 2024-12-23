use super::span::{Span, UnSpan};
use super::*;
use lalrpop_util::ParseError;
use util;

// Expressions
#[test]
fn test_parse_literal() {
    let parser = grammar::ExpressionParser::new();

    // Ok IntLiteral
    assert!(parser.parse("nothing").unwrap().unspanned() == ast::Expression::Nothing(None));
    assert!(parser.parse("4").unwrap().unspanned() == ast::Expression::IntLiteral(4, None));
    assert!(parser.parse("52").unwrap().unspanned() == ast::Expression::IntLiteral(52, None));
    assert!(
        parser.parse("-1787234").unwrap().unspanned()
            == ast::Expression::IntLiteral(-1787234, None)
    );
    assert!(parser.parse("675").unwrap().unspanned() == ast::Expression::IntLiteral(675, None));
    // Err IntLiteral
    assert!(parser.parse("0527").is_err());
    assert!(parser.parse("-000343").is_err());
    // Ok FloatLiteral
    assert!(
        parser.parse("5.0").unwrap().unspanned()
            == ast::Expression::FloatLiteral(util::to_of64(5.0), None)
    );
    assert!(
        parser.parse("1.0e-9").unwrap().unspanned()
            == ast::Expression::FloatLiteral(util::to_of64(1e-9), None)
    );
    assert!(
        parser.parse("0.23124").unwrap().unspanned()
            == ast::Expression::FloatLiteral(util::to_of64(0.23124), None)
    );
    assert!(
        parser.parse("1.2222E100").unwrap().unspanned()
            == ast::Expression::FloatLiteral(util::to_of64(1.2222E100), None)
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
        parser.parse("\"hello there\"").unwrap().unspanned()
            == ast::Expression::StringLiteral("hello there".to_string(), None)
    );
    assert!(
        parser.parse("\"µß£££ç∑ 😎\"").unwrap().unspanned()
            == ast::Expression::StringLiteral("µß£££ç∑ 😎".to_string(), None)
    );
    assert!(
        parser.parse("\"\"").unwrap().unspanned()
            == ast::Expression::StringLiteral("".to_string(), None)
    );
    // Err StringLiteral
    assert!(parser.parse("\"hi there\"\"").is_err());
    assert!(parser.parse("\"bruh").is_err());
    assert!(parser.parse("no begin! \"").is_err());
}

#[test]
fn test_parse_list() {
    let parser = grammar::ExpressionParser::new();

    assert!(parser.parse("[]").unwrap().unspanned() == ast::Expression::List(vec![], None));
    assert!(
        parser.parse("[-1.0e6]").unwrap().unspanned()
            == ast::Expression::List(
                vec![ast::Expression::FloatLiteral(util::to_of64(-1.0e6), None)],
                None
            )
    );
    assert!(
        parser.parse("[4, 5]").unwrap().unspanned()
            == ast::Expression::List(
                vec![
                    ast::Expression::IntLiteral(4, None),
                    ast::Expression::IntLiteral(5, None),
                ],
                None
            )
    );
    assert!(
        parser
            .parse("[\"buh\",4,5,7.0     , \t 8, \"⏰\"]")
            .unwrap()
            .unspanned()
            == ast::Expression::List(
                vec![
                    ast::Expression::StringLiteral("buh".to_string(), None),
                    ast::Expression::IntLiteral(4, None),
                    ast::Expression::IntLiteral(5, None),
                    ast::Expression::FloatLiteral(util::to_of64(7.0), None),
                    ast::Expression::IntLiteral(8, None),
                    ast::Expression::StringLiteral("⏰".to_string(), None)
                ],
                None
            )
    );
    // parser doesn't do type checking
    assert!(
        parser
            .parse(r#"[1, "wow ಣ", 1.0, (2), [46, 47, -9.85], (-52, )]"#)
            .unwrap()
            .unspanned()
            == ast::Expression::List(
                vec![
                    ast::Expression::IntLiteral(1, None),
                    ast::Expression::StringLiteral("wow ಣ".to_string(), None),
                    ast::Expression::FloatLiteral(util::to_of64(1.0), None),
                    ast::Expression::IntLiteral(2, None),
                    ast::Expression::List(
                        vec![
                            ast::Expression::IntLiteral(46, None),
                            ast::Expression::IntLiteral(47, None),
                            ast::Expression::FloatLiteral(util::to_of64(-9.85), None),
                        ],
                        None
                    ),
                    ast::Expression::Tuple(vec![ast::Expression::IntLiteral(-52, None)], None)
                ],
                None
            )
    );
    assert!(
        parser.parse("[x, 4]").unwrap().unspanned()
            == ast::Expression::List(
                vec![
                    ast::Expression::Identifier("x".to_string(), None),
                    ast::Expression::IntLiteral(4, None)
                ],
                None
            )
    );

    assert!(parser.parse("[,]").is_err());
    assert!(parser.parse("[,7]").is_err());
    assert!(parser.parse("[7,]").is_err());
    assert!(parser.parse("[4, 5,]").is_err());
    assert!(parser.parse("[4, -6").is_err());
    assert!(parser.parse("x, 7.0, ]").is_err());
    assert!(parser.parse("[").is_err());
    assert!(parser.parse("]").is_err());
}

#[test]
fn test_parse_tuple() {
    let parser = grammar::ExpressionParser::new();

    assert!(parser.parse("(-4)").unwrap().unspanned() == ast::Expression::IntLiteral(-4, None));
    assert!(
        parser.parse("(-4,)").unwrap().unspanned()
            == ast::Expression::Tuple(vec![ast::Expression::IntLiteral(-4, None)], None)
    );
    assert!(
        parser.parse("(5, 6, )").unwrap().unspanned()
            == ast::Expression::Tuple(
                vec![
                    ast::Expression::IntLiteral(5, None),
                    ast::Expression::IntLiteral(6, None)
                ],
                None
            )
    );
    assert!(
        parser.parse("(3, -7.25)").unwrap().unspanned()
            == ast::Expression::Tuple(
                vec![
                    ast::Expression::IntLiteral(3, None),
                    ast::Expression::FloatLiteral(util::to_of64(-7.25), None)
                ],
                None
            )
    );

    assert!(parser.parse("(").is_err());
    assert!(parser.parse(")").is_err());
    assert!(parser.parse("(4, 6, \"yah!\"").is_err());
    assert!(parser.parse("5, 6, 3)").is_err());
}

#[test]
fn test_parse_record_expr() {
    let parser = grammar::ExpressionParser::new();

    assert!(
        parser.parse("{field: Field.feeld}").unwrap().unspanned()
            == ast::Expression::Record(
                vec![(
                    "field".to_string(),
                    ast::Expression::Projection(
                        Box::new(ast::Expression::Identifier("Field".to_string(), None)),
                        "feeld".to_string(),
                        None
                    ),
                    None
                )],
                None
            )
    );
    assert!(
        parser
            .parse(
                "{
        bint: 3,
        jint: [2],
        cidnt: (1),
        lint: (\"hi\",)
    }"
            )
            .unwrap()
            .unspanned()
            == ast::Expression::Record(
                vec![
                    (
                        "bint".to_string(),
                        ast::Expression::IntLiteral(3, None),
                        None
                    ),
                    (
                        "jint".to_string(),
                        ast::Expression::List(vec![ast::Expression::IntLiteral(2, None)], None),
                        None
                    ),
                    (
                        "cidnt".to_string(),
                        ast::Expression::IntLiteral(1, None),
                        None
                    ),
                    (
                        "lint".to_string(),
                        ast::Expression::Tuple(
                            vec![ast::Expression::StringLiteral("hi".to_string(), None)],
                            None
                        ),
                        None
                    )
                ],
                None
            )
    );
    assert!(
        parser.parse("({one: 2, three: 4})").unwrap().unspanned()
            == ast::Expression::Record(
                vec![
                    (
                        "one".to_string(),
                        ast::Expression::IntLiteral(2, None),
                        None
                    ),
                    (
                        "three".to_string(),
                        ast::Expression::IntLiteral(4, None),
                        None
                    )
                ],
                None
            )
    );

    assert!(
        parser.parse("({one:2,three:4},)").unwrap().unspanned()
            == ast::Expression::Tuple(
                vec![ast::Expression::Record(
                    vec![
                        (
                            "one".to_string(),
                            ast::Expression::IntLiteral(2, None),
                            None
                        ),
                        (
                            "three".to_string(),
                            ast::Expression::IntLiteral(4, None),
                            None
                        )
                    ],
                    None
                )],
                None
            )
    );

    assert!(parser.parse("{}").is_err());
    assert!(parser.parse("{super(pub): 4}").is_err());
    assert!(parser.parse("{super: 4,}").is_err());
    assert!(parser.parse("{int: 4}").is_err());
    assert!(parser.parse("{4: thing}").is_err());
    assert!(parser.parse("unclosed: curly}").is_err());
    assert!(parser.parse("{one: two three: four}").is_err());
}

#[test]
fn test_parse_identifier() {
    let parser = grammar::ExpressionParser::new();

    assert!(
        parser.parse("x").unwrap().unspanned()
            == ast::Expression::Identifier("x".to_string(), None)
    );
    assert!(
        parser.parse("identif").unwrap().unspanned()
            == ast::Expression::Identifier("identif".to_string(), None)
    );
    assert!(
        parser.parse("hElO_").unwrap().unspanned()
            == ast::Expression::Identifier("hElO_".to_string(), None)
    );
    assert!(
        parser.parse("_a0001").unwrap().unspanned()
            == ast::Expression::Identifier("_a0001".to_string(), None)
    );
    assert!(
        parser.parse("Hello").unwrap().unspanned()
            == ast::Expression::Identifier("Hello".to_string(), None)
    );
    assert!(
        parser.parse("__Option").unwrap().unspanned()
            == ast::Expression::Identifier("__Option".to_string(), None)
    );
    assert!(
        parser.parse("Ty6_Var68__iant_").unwrap().unspanned()
            == ast::Expression::Identifier("Ty6_Var68__iant_".to_string(), None)
    );
    assert!(
        parser.parse("___01").unwrap().unspanned()
            == ast::Expression::Identifier("___01".to_string(), None)
    );
    assert!(
        parser.parse("___").unwrap().unspanned()
            == ast::Expression::Identifier("___".to_string(), None)
    );
    assert!(
        parser.parse("(<)").unwrap().unspanned()
            == ast::Expression::BinaryOp(ast::BinaryOp::Lt, None)
    );
    assert!(
        parser.parse("(+)").unwrap().unspanned()
            == ast::Expression::BinaryOp(ast::BinaryOp::Add, None)
    );
    assert!(
        parser.parse("(//)").unwrap().unspanned()
            == ast::Expression::BinaryOp(ast::BinaryOp::FloorDiv, None)
    );

    assert!(parser.parse("string").is_err());
    assert!(parser.parse("with").is_err());
    assert!(parser.parse("int").is_err());
    assert!(parser.parse("<").is_err());
    assert!(parser.parse("(-").is_err());
    assert!(parser.parse("a*").is_err());
    assert!(parser.parse("//)").is_err());
    assert!(parser.parse("yel⏰o").is_err());
    assert!(parser.parse("31232abcd").is_err());
    assert!(parser.parse("Hel)lo").is_err());
    assert!(parser.parse("31232_AA").is_err());
    assert!(parser.parse("_Yel⏰o").is_err());
    assert!(parser.parse("aபாதை").is_err());
}

#[test]
fn test_parse_enum_variant() {
    let parser = grammar::ExpressionParser::new();

    assert!(
        parser.parse("Card.King").unwrap().unspanned()
            == ast::Expression::Projection(
                Box::new(ast::Expression::Identifier("Card".to_string(), None)),
                "King".to_string(),
                None
            )
    );

    assert!(
        parser.parse("a.b.c").unwrap().unspanned()
            == ast::Expression::Projection(
                Box::new(ast::Expression::Projection(
                    Box::new(ast::Expression::Identifier("a".to_string(), None)),
                    "b".to_string(),
                    None
                )),
                "c".to_string(),
                None,
            )
    );

    assert!(
        parser.parse("Option.Some with 4").unwrap().unspanned()
            == ast::Expression::EnumVariant {
                enum_id: "Option".to_string(),
                variant: "Some".to_string(),
                field: Box::new(ast::Expression::IntLiteral(4, None)),
                span: None
            }
    );

    // I'm thinking that
    assert!(
        parser.parse("(Thing.thing with 3)").unwrap().unspanned()
            == ast::Expression::EnumVariant {
                enum_id: "Thing".to_string(),
                variant: "thing".to_string(),
                field: Box::new(ast::Expression::IntLiteral(3, None)),
                span: None,
            }
    );

    assert!(
        parser
            .parse(
                "Tree.Node with (
            (Tree.Node with (Tree.Leaf, Tree.Leaf, -2.5)),
            Tree.Leaf,
            7
        )"
            )
            .unwrap()
            .unspanned()
            == ast::Expression::EnumVariant {
                enum_id: "Tree".to_string(),
                variant: "Node".to_string(),
                span: None,
                field: Box::new(ast::Expression::Tuple(
                    vec![
                        ast::Expression::EnumVariant {
                            enum_id: "Tree".to_string(),
                            variant: "Node".to_string(),
                            span: None,
                            field: Box::new(ast::Expression::Tuple(
                                vec![
                                    ast::Expression::Projection(
                                        Box::new(ast::Expression::Identifier(
                                            "Tree".to_string(),
                                            None
                                        )),
                                        "Leaf".to_string(),
                                        None
                                    ),
                                    ast::Expression::Projection(
                                        Box::new(ast::Expression::Identifier(
                                            "Tree".to_string(),
                                            None
                                        )),
                                        "Leaf".to_string(),
                                        None
                                    ),
                                    ast::Expression::FloatLiteral(util::to_of64(-2.5), None)
                                ],
                                None
                            ))
                        },
                        ast::Expression::Projection(
                            Box::new(ast::Expression::Identifier("Tree".to_string(), None)),
                            "Leaf".to_string(),
                            None
                        ),
                        ast::Expression::IntLiteral(7, None),
                    ],
                    None
                ))
            }
    );
    assert!(
        parser
            .parse("Listy.Listy with [1, \"hell⏰\"]")
            .unwrap()
            .unspanned()
            == ast::Expression::EnumVariant {
                enum_id: "Listy".to_string(),
                variant: "Listy".to_string(),
                span: None,
                field: Box::new(ast::Expression::List(
                    vec![
                        ast::Expression::IntLiteral(1, None),
                        ast::Expression::StringLiteral("hell⏰".to_string(), None),
                    ],
                    None
                ))
            }
    );
    assert!(
        parser
            .parse("Tupy.MaybeTuple with (-5.2)")
            .unwrap()
            .unspanned()
            == ast::Expression::EnumVariant {
                enum_id: "Tupy".to_string(),
                variant: "MaybeTuple".to_string(),
                span: None,
                field: Box::new(ast::Expression::FloatLiteral(util::to_of64(-5.2), None))
            }
    );

    // Missing parenthesis
    assert!(parser
        .parse(
            "Tree.Node with (
            (Tree.Node with (Tree.Leaf, Tree.Leaf, -2.5),
            Tree.Leaf,
            7
        )"
        )
        .is_err());
    // No projection
    assert!(parser.parse("Listy with [1, \"hell⏰\"])").is_err());
    // Trailing unmatched parenthesis
    assert!(parser.parse("Listy.Listy with [1, \"hell⏰\"])").is_err());

    assert!(
        parser.parse("x.y").unwrap().unspanned()
            == ast::Expression::Projection(
                Box::new(ast::Expression::Identifier("x".to_string(), None)),
                "y".to_string(),
                None
            )
    );
    assert!(parser.parse("0xy.var").is_err());
    assert!(
        parser.parse("xy0.__xy").unwrap().unspanned()
            == ast::Expression::Projection(
                Box::new(ast::Expression::Identifier("xy0".to_string(), None)),
                "__xy".to_string(),
                None
            )
    );
    assert!(
        parser.parse("__9._a5").unwrap().unspanned()
            == ast::Expression::Projection(
                Box::new(ast::Expression::Identifier("__9".to_string(), None)),
                "_a5".to_string(),
                None
            )
    );
    assert!(parser.parse("xs*.bruh").is_err());
    assert!(parser.parse("x.8").is_err());
    assert!(parser.parse("Yu.p with [8, 78").is_err());
    assert!(parser.parse("Option.Some int").is_err());
    assert!(parser.parse("He)i.k with 4").is_err());
    // Tokens should have at least a space between them
    // Collect spans to check collision
    assert!(parser.parse("(a_9.u8)with \"hi\"").is_err());
    assert!(matches!(
        parser.parse("  thingy.thing with\"hi\"").err().unwrap(),
        ParseError::User { error: (e, s) }
        if e.contains("Space required") && s == Span::new(15, 23)
    ));
}

#[test]
fn test_parse_func_application() {
    let parser = grammar::ExpressionParser::new();

    assert!(
        parser.parse("hello a=4.5 br=8").unwrap().unspanned()
            == ast::Expression::NamedArgsFuncApp(
                Box::new(ast::Expression::Identifier("hello".to_string(), None)),
                vec![
                    (
                        "a".to_string(),
                        ast::Expression::FloatLiteral(util::to_of64(4.5), None),
                        None
                    ),
                    ("br".to_string(), ast::Expression::IntLiteral(8, None), None)
                ],
                None
            )
    );

    assert!(
        parser.parse("X.Y 3").unwrap().unspanned()
            == ast::Expression::FuncApplication(
                Box::new(ast::Expression::Projection(
                    Box::new(ast::Expression::Identifier("X".to_string(), None)),
                    "Y".to_string(),
                    None,
                )),
                vec![ast::Expression::IntLiteral(3, None)],
                None
            )
    );
}
