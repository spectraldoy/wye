use super::ast;
use super::ast::Expression::*;
use super::ast::Statement::Expression;
use super::ast::VarWithValue;
use super::span::{Span, UnSpan};
use super::*;
use crate::test_util::to_of64;
use lalrpop_util::ParseError;
use std::collections::HashMap;

fn parse(parser: &grammar::StatementParser, inp: &'static str) -> ast::Expression {
    let out = parser.parse(inp).unwrap().unspanned();
    if let Expression(e) = out {
        e
    } else {
        panic!("Input is not an expression")
    }
}

// Expressions
#[test]
fn test_parse_literal() {
    let parser = grammar::StatementParser::new();

    // Ok IntLiteral
    assert!(parse(&parser, "nothing") == Nothing(None));
    assert!(parse(&parser, "fail") == Fail(None));
    assert!(parse(&parser, "print") == Print(None));
    assert!(parse(&parser, "4") == IntLiteral(4, None));
    assert!(parse(&parser, "52") == IntLiteral(52, None));
    assert!(parse(&parser, "-1787234") == IntLiteral(-1787234, None));
    assert!(parse(&parser, "675") == IntLiteral(675, None));
    // Err IntLiteral
    assert!(parser.parse("0527").is_err());
    assert!(parser.parse("-000343").is_err());
    // Ok FloatLiteral
    assert!(parse(&parser, "5.0") == FloatLiteral(to_of64(5.0), None));
    assert!(parse(&parser, "1.0e-9") == FloatLiteral(to_of64(1e-9), None));
    assert!(parse(&parser, "0.23124") == FloatLiteral(to_of64(0.23124), None));
    assert!(parse(&parser, "1.2222E100") == FloatLiteral(to_of64(1.2222E100), None));
    // Err FloatLiteral
    assert!(parser.parse("00.9").is_err());
    assert!(parser.parse("4.").is_err());
    assert!(parser.parse("0.5689eE2").is_err());
    assert!(parser.parse("12.888e").is_err());
    assert!(parser.parse("3.145r10").is_err());
    assert!(parser.parse("1.2.3.4").is_err());
    assert!(parser.parse("5 .0").is_err());

    assert!(parse(&parser, "\"hello there\"") == StringLiteral("hello there".to_string(), None));
    assert!(parse(&parser, "\"µß£££ç∑ 😎\"") == StringLiteral("µß£££ç∑ 😎".to_string(), None));
    assert!(parse(&parser, "\"\"") == StringLiteral("".to_string(), None));
    assert!(parser.parse("\"hi there\"\"").is_err());
    assert!(parser.parse("\"bruh").is_err());
    assert!(parser.parse("no begin! \"").is_err());
}

#[test]
fn test_parse_list() {
    let parser = grammar::StatementParser::new();

    assert!(parse(&parser, "[]") == List(vec![], None));
    assert!(parse(&parser, "[-1.0e6]") == List(vec![FloatLiteral(to_of64(-1.0e6), None)], None));
    assert!(
        parse(&parser, "[4, 5]") == List(vec![IntLiteral(4, None), IntLiteral(5, None),], None)
    );
    assert!(
        parse(&parser, "[\"buh\",4,5,7.0     , \t 8, \"⏰\"]")
            == List(
                vec![
                    StringLiteral("buh".to_string(), None),
                    IntLiteral(4, None),
                    IntLiteral(5, None),
                    FloatLiteral(to_of64(7.0), None),
                    IntLiteral(8, None),
                    StringLiteral("⏰".to_string(), None)
                ],
                None
            )
    );
    // parser doesn't do type checking
    assert!(
        parse(
            &parser,
            r#"[1, "wow ಣ", 1.0, (2), [46, 47, -9.85], (-52, )]"#
        ) == List(
            vec![
                IntLiteral(1, None),
                StringLiteral("wow ಣ".to_string(), None),
                FloatLiteral(to_of64(1.0), None),
                IntLiteral(2, None),
                List(
                    vec![
                        IntLiteral(46, None),
                        IntLiteral(47, None),
                        FloatLiteral(to_of64(-9.85), None),
                    ],
                    None
                ),
                Tuple(vec![IntLiteral(-52, None)], None)
            ],
            None
        )
    );
    assert!(
        parse(&parser, "[x, 4]")
            == List(
                vec![Identifier("x".to_string(), None), IntLiteral(4, None)],
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
    let parser = grammar::StatementParser::new();

    assert!(parse(&parser, "(-4)") == IntLiteral(-4, None));
    assert!(parse(&parser, "(-4,)") == Tuple(vec![IntLiteral(-4, None)], None));
    assert!(
        parse(&parser, "(5, 6, )") == Tuple(vec![IntLiteral(5, None), IntLiteral(6, None)], None)
    );
    assert!(
        parse(&parser, "(3, -7.25)")
            == Tuple(
                vec![IntLiteral(3, None), FloatLiteral(to_of64(-7.25), None)],
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
    let parser = grammar::StatementParser::new();

    assert!(
        parse(&parser, "{field: Field.feeld}")
            == StructRecord(
                HashMap::from([(
                    "field".to_string(),
                    (
                        Projection(
                            Box::new(Identifier("Field".to_string(), None)),
                            "feeld".to_string(),
                            None
                        ),
                        None
                    )
                )]),
                None
            )
    );
    assert!(
        parse(
            &parser,
            "{
        bint: 3,
        jint: [2],
        cidnt: (1),
        lint: (\"hi\",)
    }"
        ) == StructRecord(
            HashMap::from([
                ("bint".to_string(), (IntLiteral(3, None), None)),
                (
                    "jint".to_string(),
                    (List(vec![IntLiteral(2, None)], None), None)
                ),
                ("cidnt".to_string(), (IntLiteral(1, None), None)),
                (
                    "lint".to_string(),
                    (
                        Tuple(vec![StringLiteral("hi".to_string(), None)], None),
                        None
                    )
                )
            ]),
            None
        )
    );
    assert!(
        parse(&parser, "({one: 2, three: 4})")
            == StructRecord(
                HashMap::from([
                    ("one".to_string(), (IntLiteral(2, None), None)),
                    ("three".to_string(), (IntLiteral(4, None), None))
                ]),
                None
            )
    );

    assert!(
        parse(&parser, "({one:2,three:4},)")
            == Tuple(
                vec![StructRecord(
                    HashMap::from([
                        ("one".to_string(), (IntLiteral(2, None), None)),
                        ("three".to_string(), (IntLiteral(4, None), None))
                    ]),
                    None
                )],
                None
            )
    );
    assert!(
        parse(&parser, "{super: 4,}")
            == StructRecord(
                HashMap::from([("super".to_string(), (IntLiteral(4, None), None))]),
                None
            )
    );
    assert!(
        parse(&parser, "{|x: 4|}")
            == NominalRecord(
                HashMap::from([("x".to_string(), (IntLiteral(4, None), None))]),
                None
            )
    );

    assert!(parser.parse("{}").is_err());
    assert!(parser.parse("{||}").is_err());
    assert!(parser.parse("{super(pub): 4}").is_err());
    assert!(parser.parse("{int: 4}").is_err());
    assert!(parser.parse("{|int: 4|}").is_err());
    assert!(parser.parse("{4: thing}").is_err());
    assert!(parser.parse("unclosed: curly}").is_err());
    assert!(parser.parse("{one: two three: four}").is_err());
    assert!(parser.parse("{a: 2, a: 3}").is_err());
}

#[test]
fn test_parse_identifier() {
    let parser = grammar::StatementParser::new();

    assert!(parse(&parser, "x") == Identifier("x".to_string(), None));
    assert!(parse(&parser, "identif") == Identifier("identif".to_string(), None));
    assert!(parse(&parser, "hElO_") == Identifier("hElO_".to_string(), None));
    assert!(parse(&parser, "_a0001") == Identifier("_a0001".to_string(), None));
    assert!(parse(&parser, "Hello") == Identifier("Hello".to_string(), None));
    assert!(parse(&parser, "__Option") == Identifier("__Option".to_string(), None));
    assert!(parse(&parser, "Ty6_Var68__iant_") == Identifier("Ty6_Var68__iant_".to_string(), None));
    assert!(parse(&parser, "___01") == Identifier("___01".to_string(), None));
    assert!(parse(&parser, "___") == Identifier("___".to_string(), None));
    assert!(parse(&parser, "(<)") == BinaryOp(ast::BinaryOp::Lt, None));
    assert!(parse(&parser, "(+)") == BinaryOp(ast::BinaryOp::Add, None));
    assert!(parse(&parser, "(//)") == BinaryOp(ast::BinaryOp::FloorDiv, None));

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
    let parser = grammar::StatementParser::new();

    assert!(
        parse(&parser, "Card.King")
            == Projection(
                Box::new(Identifier("Card".to_string(), None)),
                "King".to_string(),
                None
            )
    );

    assert!(
        parse(&parser, "Card . King")
            == Projection(
                Box::new(Identifier("Card".to_string(), None)),
                "King".to_string(),
                None
            )
    );

    assert!(
        parse(&parser, "a.b.c")
            == Projection(
                Box::new(Projection(
                    Box::new(Identifier("a".to_string(), None)),
                    "b".to_string(),
                    None
                )),
                "c".to_string(),
                None,
            )
    );

    assert!(
        parse(&parser, "Option.Some with 4")
            == EnumVariant {
                enum_id: "Option".to_string(),
                variant: "Some".to_string(),
                field: Box::new(IntLiteral(4, None)),
                span: None
            }
    );

    assert!(
        parse(&parser, "hello. there")
            == Projection(
                Box::new(Identifier("hello".to_string(), None)),
                "there".to_string(),
                None,
            )
    );

    assert!(parser.parse("(hello).there").is_err());
    assert!(parser.parse("(hello foo). there").is_err());
    assert!(parser.parse("hi with 4").is_err());
    assert!(parser.parse("Option.Some.Other with 3").is_err());

    assert!(
        parse(&parser, "Option .Some with 4")
            == EnumVariant {
                enum_id: "Option".to_string(),
                variant: "Some".to_string(),
                field: Box::new(IntLiteral(4, None)),
                span: None
            }
    );

    assert!(
        parse(&parser, "(Thing.thing with 3)")
            == EnumVariant {
                enum_id: "Thing".to_string(),
                variant: "thing".to_string(),
                field: Box::new(IntLiteral(3, None)),
                span: None,
            }
    );

    assert!(
        parse(
            &parser,
            "Tree.Node with (
            (Tree.Node with (Tree.Leaf, Tree.Leaf, -2.5)),
            Tree.Leaf,
            7
        )"
        ) == EnumVariant {
            enum_id: "Tree".to_string(),
            variant: "Node".to_string(),
            span: None,
            field: Box::new(Tuple(
                vec![
                    EnumVariant {
                        enum_id: "Tree".to_string(),
                        variant: "Node".to_string(),
                        span: None,
                        field: Box::new(Tuple(
                            vec![
                                Projection(
                                    Box::new(Identifier("Tree".to_string(), None)),
                                    "Leaf".to_string(),
                                    None
                                ),
                                Projection(
                                    Box::new(Identifier("Tree".to_string(), None)),
                                    "Leaf".to_string(),
                                    None
                                ),
                                FloatLiteral(to_of64(-2.5), None)
                            ],
                            None
                        ))
                    },
                    Projection(
                        Box::new(Identifier("Tree".to_string(), None)),
                        "Leaf".to_string(),
                        None
                    ),
                    IntLiteral(7, None),
                ],
                None
            ))
        }
    );
    assert!(
        parse(&parser, "Listy.Listy with [1, \"hell⏰\"]")
            == EnumVariant {
                enum_id: "Listy".to_string(),
                variant: "Listy".to_string(),
                span: None,
                field: Box::new(List(
                    vec![
                        IntLiteral(1, None),
                        StringLiteral("hell⏰".to_string(), None),
                    ],
                    None
                ))
            }
    );
    assert!(
        parse(&parser, "Tupy.MaybeTuple with (-5.2)")
            == EnumVariant {
                enum_id: "Tupy".to_string(),
                variant: "MaybeTuple".to_string(),
                span: None,
                field: Box::new(FloatLiteral(to_of64(-5.2), None))
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
        parse(&parser, "x.y")
            == Projection(
                Box::new(Identifier("x".to_string(), None)),
                "y".to_string(),
                None
            )
    );
    assert!(parser.parse("0xy.var").is_err());
    assert!(
        parse(&parser, "xy0.__xy")
            == Projection(
                Box::new(Identifier("xy0".to_string(), None)),
                "__xy".to_string(),
                None
            )
    );
    assert!(
        parse(&parser, "__9._a5")
            == Projection(
                Box::new(Identifier("__9".to_string(), None)),
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
    let parser = grammar::StatementParser::new();

    assert!(
        parse(&parser, "hello a=4.5 br=8")
            == NamedArgsFuncApp(
                Box::new(Identifier("hello".to_string(), None)),
                vec![
                    ("a".to_string(), FloatLiteral(to_of64(4.5), None), None),
                    ("br".to_string(), IntLiteral(8, None), None)
                ],
                None
            )
    );

    assert!(
        parse(&parser, "X.Y 3")
            == FuncApplication(
                Box::new(Projection(
                    Box::new(Identifier("X".to_string(), None)),
                    "Y".to_string(),
                    None,
                )),
                vec![IntLiteral(3, None)],
                None
            )
    );

    assert!(
        parse(&parser, "f g")
            == FuncApplication(
                Box::new(Identifier("f".to_string(), None)),
                vec![Identifier("g".to_string(), None)],
                None
            )
    );

    assert!(
        parse(&parser, "f -7.9")
            == FuncApplication(
                Box::new(Identifier("f".to_string(), None)),
                vec![FloatLiteral(to_of64(-7.9), None)],
                None
            )
    );

    assert!(
        parse(&parser, "f -4 2")
            == FuncApplication(
                Box::new(Identifier("f".to_string(), None)),
                vec![IntLiteral(-4, None), IntLiteral(2, None)],
                None
            )
    );

    assert!(
        parse(&parser, "(g 5)")
            == FuncApplication(
                Box::new(Identifier("g".to_string(), None)),
                vec![IntLiteral(5, None)],
                None
            )
    );

    assert!(
        parser
            .parse("(g 4 \"hi\" (f bruh = 2))")
            .unwrap()
            .unspanned()
            == Expression(FuncApplication(
                Box::new(Identifier("g".to_string(), None)),
                vec![
                    IntLiteral(4, None),
                    StringLiteral("hi".to_string(), None),
                    NamedArgsFuncApp(
                        Box::new(Identifier("f".to_string(), None)),
                        vec![("bruh".to_string(), IntLiteral(2, None), None),],
                        None,
                    )
                ],
                None
            ))
    );
    assert!(
        parse(&parser, "print 4")
            == FuncApplication(Box::new(Print(None)), vec![IntLiteral(4, None)], None,)
    );
    assert!(
        parse(&parser, "(+) 4")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                vec![IntLiteral(4, None)],
                None,
            )
    );

    assert!(
        parse(&parser, "(f 2) (g 3)")
            == FuncApplication(
                Box::new(FuncApplication(
                    Box::new(Identifier("f".to_string(), None)),
                    vec![IntLiteral(2, None)],
                    None,
                )),
                vec![FuncApplication(
                    Box::new(Identifier("g".to_string(), None)),
                    vec![IntLiteral(3, None)],
                    None,
                )],
                None
            )
    );

    assert!(
        parser
            .parse("[f g.e.t, x.y -5.6, Option.Some with 4]")
            .unwrap()
            .unspanned()
            == Expression(List(
                vec![
                    FuncApplication(
                        Box::new(Identifier("f".to_string(), None)),
                        vec![Projection(
                            Box::new(Projection(
                                Box::new(Identifier("g".to_string(), None)),
                                "e".to_string(),
                                None,
                            )),
                            "t".to_string(),
                            None
                        )],
                        None,
                    ),
                    FuncApplication(
                        Box::new(Projection(
                            Box::new(Identifier("x".to_string(), None)),
                            "y".to_string(),
                            None,
                        )),
                        vec![FloatLiteral(to_of64(-5.6), None)],
                        None
                    ),
                    EnumVariant {
                        enum_id: "Option".to_string(),
                        variant: "Some".to_string(),
                        field: Box::new(IntLiteral(4, None)),
                        span: None
                    }
                ],
                None
            ))
    );

    assert!(parser.parse("f Option.Some with 4").is_err());
    assert!(parser.parse("f (Option.Some with 4)").is_ok());
    // As far as parsing is concerned, this is not an error.
    assert!(
        parse(&parser, "4 g")
            == FuncApplication(
                Box::new(IntLiteral(4, None)),
                vec![Identifier("g".to_string(), None)],
                None,
            )
    );
    assert!(parser.parse("f(8)").is_err());
    assert!(parser.parse("f(g=8)").is_err());
    assert!(parser.parse("f (g=8)").is_err());
    assert!(
        parse(&parser, "f g=(8)")
            == NamedArgsFuncApp(
                Box::new(Identifier("f".to_string(), None)),
                vec![("g".to_string(), IntLiteral(8, None), None)],
                None,
            )
    );
    assert!(parser.parse("f.g=3").is_err());
    assert!(parser.parse("f g=").is_err());
    assert!(parser.parse("f\"hi\" 3").is_err());
}

#[test]
fn test_parse_infix_binary_op() {
    let parser = grammar::StatementParser::new();

    assert!(
        parse(&parser, "4 + 5 * 6")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                vec![
                    IntLiteral(4, None),
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Mult, None)),
                        vec![IntLiteral(5, None), IntLiteral(6, None)],
                        None
                    )
                ],
                None
            )
    );
    assert!(parser.parse("* 3 4").is_err());
    assert!(
        parse(&parser, "(*) 3 4")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Mult, None)),
                vec![IntLiteral(3, None), IntLiteral(4, None)],
                None,
            )
    );
    assert!(
        parse(&parser, "1 * 5 + 2 / 4")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                vec![
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Mult, None)),
                        vec![IntLiteral(1, None), IntLiteral(5, None),],
                        None,
                    ),
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Div, None)),
                        vec![IntLiteral(2, None), IntLiteral(4, None),],
                        None
                    )
                ],
                None
            )
    );

    assert!(
        parse(&parser, "1.5 == 2.5 == 3.5")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Eq, None)),
                vec![
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Eq, None)),
                        vec![
                            FloatLiteral(to_of64(1.5), None),
                            FloatLiteral(to_of64(2.5), None),
                        ],
                        None
                    ),
                    FloatLiteral(to_of64(3.5), None)
                ],
                None
            )
    );

    assert!(
        parse(&parser, "(l1 :: l2) * 5")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Mult, None)),
                vec![
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Cons, None)),
                        vec![
                            Identifier("l1".to_string(), None),
                            Identifier("l2".to_string(), None),
                        ],
                        None,
                    ),
                    IntLiteral(5, None),
                ],
                None,
            )
    );

    assert!(
        parse(&parser, "a < b + 5")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Lt, None)),
                vec![
                    Identifier("a".to_string(), None),
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                        vec![Identifier("b".to_string(), None), IntLiteral(5, None),],
                        None,
                    ),
                ],
                None,
            )
    );
    assert!(
        parse(&parser, "(a >= b) + 5")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                vec![
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Geq, None)),
                        vec![
                            Identifier("a".to_string(), None),
                            Identifier("b".to_string(), None),
                        ],
                        None,
                    ),
                    IntLiteral(5, None),
                ],
                None,
            )
    );

    assert!(parse(&parser, "(<=)") == BinaryOp(ast::BinaryOp::Leq, None));
    assert!(
        parse(&parser, "(//) 4")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::FloorDiv, None)),
                vec![IntLiteral(4, None)],
                None,
            )
    );
    assert!(
        parse(&parser, "((::) 4) - ((-) -6 \"hi\")")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Sub, None)),
                vec![
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Cons, None)),
                        vec![IntLiteral(4, None)],
                        None,
                    ),
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Sub, None)),
                        vec![IntLiteral(-6, None), StringLiteral("hi".to_string(), None),],
                        None,
                    ),
                ],
                None,
            )
    );

    assert!(
        parse(&parser, "(::) 4  ((-) -6 \"hi\")")
            == FuncApplication(
                Box::new(BinaryOp(ast::BinaryOp::Cons, None)),
                vec![
                    IntLiteral(4, None),
                    FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Sub, None)),
                        vec![IntLiteral(-6, None), StringLiteral("hi".to_string(), None),],
                        None,
                    ),
                ],
                None,
            )
    );

    assert!(parser.parse("(::) 4 - ((-) -6 \"hi\")").is_err());
    assert!(parser.parse("a + f b").is_err());
    assert!(parser.parse("1 >").is_err());
    assert!(parser.parse("pam f a :: lst").is_err());
}

#[test]
fn test_parse_let() {
    let parser = grammar::StatementParser::new();

    assert!(
        parse(&parser, "let x = 4")
            == Let(
                VarWithValue {
                    name: ("x".to_string(), None),
                    args: vec![],
                    rec: false,
                    expr: Box::new(IntLiteral(4, None))
                },
                None,
                None,
            )
    );
    assert!(
        parse(&parser, "let y z = (x 4)")
            == Let(
                VarWithValue {
                    name: ("y".to_string(), None),
                    args: vec![("z".to_string(), None)],
                    rec: false,
                    expr: Box::new(FuncApplication(
                        Box::new(Identifier("x".to_string(), None)),
                        vec![IntLiteral(4, None)],
                        None
                    )),
                },
                None,
                None,
            )
    );
    assert!(
        parse(&parser, "let rec mu = 3")
            == Let(
                VarWithValue {
                    name: ("mu".to_string(), None),
                    args: vec![],
                    rec: true,
                    expr: Box::new(IntLiteral(3, None))
                },
                None,
                None
            )
    );
    assert!(
        parse(&parser, "let x y = (+) y 4")
            == Let(
                VarWithValue {
                    name: ("x".to_string(), None),
                    args: vec![("y".to_string(), None)],
                    rec: false,
                    expr: Box::new(FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                        vec![Identifier("y".to_string(), None), IntLiteral(4, None),],
                        None,
                    ))
                },
                None,
                None,
            )
    );
    assert!(
        parse(&parser, "let plus_4 x = x + 4")
            == Let(
                VarWithValue {
                    name: ("plus_4".to_string(), None),
                    args: vec![("x".to_string(), None)],
                    rec: false,
                    expr: Box::new(FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                        vec![Identifier("x".to_string(), None), IntLiteral(4, None),],
                        None,
                    ))
                },
                None,
                None,
            )
    );

    assert!(
        parse(&parser, "let x = 4 in x + 9")
            == Let(
                VarWithValue {
                    name: ("x".to_string(), None),
                    args: vec![],
                    rec: false,
                    expr: Box::new(IntLiteral(4, None)),
                },
                Some(Box::new(FuncApplication(
                    Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                    vec![Identifier("x".to_string(), None), IntLiteral(9, None),],
                    None,
                ))),
                None,
            )
    );

    assert!(
        parse(&parser, "let x = 5 in { a: x, b: 8 }")
            == Let(
                VarWithValue {
                    name: ("x".to_string(), None),
                    args: vec![],
                    rec: false,
                    expr: Box::new(IntLiteral(5, None)),
                },
                Some(Box::new(StructRecord(
                    HashMap::from([
                        ("a".to_string(), (Identifier("x".to_string(), None), None)),
                        ("b".to_string(), (IntLiteral(8, None), None)),
                    ]),
                    None,
                ))),
                None,
            )
    );

    assert!(
        parse(&parser, "let rec pam f lst = pam f (a :: lst)")
            == Let(
                VarWithValue {
                    name: ("pam".to_string(), None),
                    args: vec![("f".to_string(), None), ("lst".to_string(), None),],
                    rec: true,
                    expr: Box::new(FuncApplication(
                        Box::new(Identifier("pam".to_string(), None)),
                        vec![
                            Identifier("f".to_string(), None),
                            FuncApplication(
                                Box::new(BinaryOp(ast::BinaryOp::Cons, None)),
                                vec![
                                    Identifier("a".to_string(), None),
                                    Identifier("lst".to_string(), None),
                                ],
                                None,
                            )
                        ],
                        None,
                    ))
                },
                None,
                None,
            )
    );
    // multi-let
    assert!(
        parse(
            &parser,
            "let x = 4 in let y z = x + 4 in let z = y in let x = a + 8 + b"
        ) == Let(
            VarWithValue {
                name: ("x".to_string(), None),
                args: vec![],
                rec: false,
                expr: Box::new(IntLiteral(4, None)),
            },
            Some(Box::new(Let(
                VarWithValue {
                    name: ("y".to_string(), None),
                    args: vec![("z".to_string(), None)],
                    rec: false,
                    expr: Box::new(FuncApplication(
                        Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                        vec![Identifier("x".to_string(), None), IntLiteral(4, None),],
                        None,
                    ))
                },
                Some(Box::new(Let(
                    VarWithValue {
                        name: ("z".to_string(), None),
                        args: vec![],
                        rec: false,
                        expr: Box::new(Identifier("y".to_string(), None))
                    },
                    Some(Box::new(Let(
                        VarWithValue {
                            name: ("x".to_string(), None),
                            args: vec![],
                            rec: false,
                            expr: Box::new(FuncApplication(
                                Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                                vec![
                                    FuncApplication(
                                        Box::new(BinaryOp(ast::BinaryOp::Add, None)),
                                        vec![
                                            Identifier("a".to_string(), None),
                                            IntLiteral(8, None),
                                        ],
                                        None,
                                    ),
                                    Identifier("b".to_string(), None),
                                ],
                                None,
                            ))
                        },
                        None,
                        None,
                    ))),
                    None,
                ))),
                None,
            ))),
            None,
        )
    );

    assert!(parser.parse("let x = 4 in x + 9 in 3").is_err());
    assert!(parser.parse("let x y = ").is_err());
    assert!(parser.parse("let x let = 5 + x").is_err());
    assert!(parser.parse("let a = let b = 5").is_err());
    assert!(parser.parse("let a = b = 5").is_err());
    assert!(parser.parse("let func (x) y = 4").is_err());
    assert!(parser.parse("let x = int").is_err());
    assert!(parser.parse("let = 4").is_err());
}
