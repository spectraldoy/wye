use super::*;
use ordered_float::OrderedFloat;

// Expressions

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
    assert!(parser.parse("5 .0").is_err());
    // Ok StringLiteral
    assert!(parser.parse("\"hello there\"").unwrap() == ast::Expression::StringLiteral(String::from("hello there")));
    assert!(parser.parse("\"µß£££ç∑ 😎\"").unwrap() == ast::Expression::StringLiteral(String::from("µß£££ç∑ 😎")));
    assert!(parser.parse("\"\"").unwrap() == ast::Expression::StringLiteral(String::from("")));
    // Err StringLiteral
    assert!(parser.parse("\"hi there\"\"").is_err());
    assert!(parser.parse("\"bruh").is_err());
    assert!(parser.parse("no begin! \"").is_err());
}

#[test]
fn test_parse_list() {
    let parser = grammar::ExpressionParser::new();

    // Ok
    assert!(parser.parse("[]").unwrap() == ast::Expression::List(vec![]));
    assert!(parser.parse("[-1.0e6]").unwrap() == ast::Expression::List(vec![
        ast::Expression::FloatLiteral(OrderedFloat(-1_000_000.0))
    ]));
    assert!(parser.parse("[4, 5]").unwrap() == ast::Expression::List(vec![
        ast::Expression::IntegerLiteral(4),
        ast::Expression::IntegerLiteral(5),
    ]));
    assert!(parser.parse("[\"buh\",4,5,7.0     , \t 8, \"⏰\"]").unwrap() == ast::Expression::List(vec![
        ast::Expression::StringLiteral(String::from("buh")),
        ast::Expression::IntegerLiteral(4),
        ast::Expression::IntegerLiteral(5),
        ast::Expression::FloatLiteral(OrderedFloat(7.0)),
        ast::Expression::IntegerLiteral(8),
        ast::Expression::StringLiteral(String::from("⏰"))
    ]));
    // parser doesn't do type checking
    assert!(parser.parse(r#"[1, "wow ಣ", 1.0, (2), [46, 47, -9.85], (-52, )]"#).unwrap() == ast::Expression::List(vec![
        ast::Expression::IntegerLiteral(1),
        ast::Expression::StringLiteral(String::from("wow ಣ")),
        ast::Expression::FloatLiteral(OrderedFloat(1.0)),
        ast::Expression::IntegerLiteral(2),
        ast::Expression::List(vec![
            ast::Expression::IntegerLiteral(46),
            ast::Expression::IntegerLiteral(47),
            ast::Expression::FloatLiteral(OrderedFloat(-9.85)),
        ]),
        ast::Expression::Tuple(vec![ast::Expression::IntegerLiteral(-52)])
    ]));
    assert!(parser.parse("[x, 4]").unwrap() == ast::Expression::List(vec![
        ast::Expression::Identifier(String::from("x")),
        ast::Expression::IntegerLiteral(4)
    ]));

    // Err
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

    // Ok
    assert!(parser.parse("(-4)").unwrap() == ast::Expression::IntegerLiteral(-4));
    assert!(parser.parse("(-4,)").unwrap() == ast::Expression::Tuple(vec![
        ast::Expression::IntegerLiteral(-4)
    ]));
    assert!(parser.parse("(5, 6, )").unwrap() == ast::Expression::Tuple(vec![
        ast::Expression::IntegerLiteral(5),
        ast::Expression::IntegerLiteral(6)
    ]));
    assert!(parser.parse("(3, -7.25)").unwrap() == ast::Expression::Tuple(vec![
        ast::Expression::IntegerLiteral(3),
        ast::Expression::FloatLiteral(OrderedFloat(-7.25))
    ]));

    // Err
    assert!(parser.parse("(").is_err());
    assert!(parser.parse(")").is_err());
    assert!(parser.parse("(4, 6, \"yah!\"").is_err());
    assert!(parser.parse("5, 6, 3)").is_err());
}

#[test]
fn test_parse_identifier_type_variant() {
    let parser = grammar::ExpressionParser::new();
    // Ok identifier
    assert!(parser.parse("x").unwrap() == ast::Expression::Identifier(String::from("x")));
    assert!(parser.parse("identif").unwrap() == ast::Expression::Identifier(String::from("identif")));
    assert!(parser.parse("hElO_").unwrap() == ast::Expression::Identifier(String::from("hElO_")));
    assert!(parser.parse("_a0001").unwrap() == ast::Expression::Identifier(String::from("_a0001")));
    assert!(parser.parse("Hello").unwrap() == ast::Expression::Identifier(String::from("Hello")));
    assert!(parser.parse("__Option").unwrap() == ast::Expression::Identifier(String::from("__Option")));
    assert!(parser.parse("Ty6_Var68__iant_").unwrap() == ast::Expression::Identifier(String::from("Ty6_Var68__iant_")));
    assert!(parser.parse("___01").unwrap() == ast::Expression::Identifier(String::from("___01")));
    assert!(parser.parse("___").unwrap() == ast::Expression::Identifier(String::from("___")));
    assert!(parser.parse("(<)").unwrap() == ast::Expression::BuiltinOp(ast::Operation::Lt));
    assert!(parser.parse("(+)").unwrap() == ast::Expression::BuiltinOp(ast::Operation::Add));
    assert!(parser.parse("(//)").unwrap() == ast::Expression::BuiltinOp(ast::Operation::FloorDiv));
    // Err identifier
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
    // Ok type variant
    assert!(parser.parse("Some with 4").unwrap() == ast::Expression::TypeVariant(
        String::from("Some"), Box::new(ast::Expression::IntegerLiteral(4))
    ));
    assert!(parser.parse("(Thing with 4)").unwrap() == ast::Expression::TypeVariant(
        String::from("Thing"), Box::new(ast::Expression::IntegerLiteral(4))
    ));
    assert!(parser.parse("
        Node with (
            (Node with (Leaf, Leaf, 4)),
            Leaf,
            7
        )").unwrap() == 
        ast::Expression::TypeVariant(String::from("Node"), Box::new(ast::Expression::Tuple(vec![
            ast::Expression::TypeVariant(String::from("Node"), Box::new(ast::Expression::Tuple(vec![
                ast::Expression::Identifier(String::from("Leaf")),
                ast::Expression::Identifier(String::from("Leaf")),
                ast::Expression::IntegerLiteral(4)
            ]))),
            ast::Expression::Identifier(String::from("Leaf")),
            ast::Expression::IntegerLiteral(7)
        ])))
    );
    assert!(parser.parse("Listy with [1, \"hell⏰\"]").unwrap() == ast::Expression::TypeVariant(String::from("Listy"), Box::new(
        ast::Expression::List(vec![
            ast::Expression::IntegerLiteral(1),
            ast::Expression::StringLiteral(String::from("hell⏰"))
        ])
    )));
    assert!(parser.parse("Bruh with (-5.2)").unwrap() == ast::Expression::TypeVariant(
        String::from("Bruh"), Box::new(ast::Expression::FloatLiteral(OrderedFloat(-5.2)))
    ));
    assert!(parser.parse("Bruh with (-5.2,)").unwrap() == ast::Expression::TypeVariant(
        String::from("Bruh"),
        Box::new(ast::Expression::Tuple(vec![
            ast::Expression::FloatLiteral(OrderedFloat(-5.2))
        ]))
    ));
    // Err type variant
    assert!(parser.parse("Yup with [8, 78").is_err());
    assert!(parser.parse("Some int").is_err());
    assert!(parser.parse("He)i with 4").is_err());
    assert!(parser.parse("(__9)with \"hi\"").is_err());
    assert!(parser.parse("  thingy with\"hi\"").is_err());
}

#[test]
fn test_parse_function_application() {
    let parser = grammar::ExpressionParser::new();

    // Ok function application
    assert!(parser.parse("f g").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier(String::from("f"))),
        Box::new(ast::Expression::Identifier(String::from("g")))
    ));
    assert!(parser.parse("f -7.9").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier(String::from("f"))),
        Box::new(ast::Expression::FloatLiteral(OrderedFloat(-7.9)))
    ));
    assert!(parser.parse("f 4 2").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::IntegerLiteral(4))
        )),
        Box::new(ast::Expression::IntegerLiteral(2))
    ));
    assert!(parser.parse("(g 5)").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier(String::from("g"))),
        Box::new(ast::Expression::IntegerLiteral(5))
    ));
    assert!(parser.parse("(g 4 \"hi\" (f 2))").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier(String::from("g"))),
                Box::new(ast::Expression::IntegerLiteral(4))
            )),
            Box::new(ast::Expression::StringLiteral(String::from("hi")))
        )),
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::IntegerLiteral(2))
        ))
    ));
    assert!(parser.parse("(+) 4").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
        Box::new(ast::Expression::IntegerLiteral(4))
    ));
    assert!(parser.parse("(::) 4 ((-) -6 \"hi\")").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::Cons)),
            Box::new(ast::Expression::IntegerLiteral(4))
        )),
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::BuiltinOp(ast::Operation::Subtract)),
                Box::new(ast::Expression::IntegerLiteral(-6))
            )),
            Box::new(ast::Expression::StringLiteral(String::from("hi")))
        ))
    ));
    assert!(parser.parse("-9 :: [3, 4]").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::Cons)),
            Box::new(ast::Expression::IntegerLiteral(-9))
        )),
        Box::new(ast::Expression::List(vec![
            ast::Expression::IntegerLiteral(3),
            ast::Expression::IntegerLiteral(4)
        ]))
    ));
    assert!(parser.parse("a + b").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
            Box::new(ast::Expression::Identifier(String::from("a")))
        )),
        Box::new(ast::Expression::Identifier(String::from("b")))
    ));
    assert!(parser.parse("a//(b *6)").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::FloorDiv)),
            Box::new(ast::Expression::Identifier(String::from("a")))
        )),
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
                Box::new(ast::Expression::Identifier(String::from("b")))
            )),
            Box::new(ast::Expression::IntegerLiteral(6))
        ))
    ));
    assert!(parser.parse("((a<b)   *c)").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::FuncApplication(
                    Box::new(ast::Expression::BuiltinOp(ast::Operation::Lt)),
                    Box::new(ast::Expression::Identifier(String::from("a")))
                )),
                Box::new(ast::Expression::Identifier(String::from("b")))
            ))
        )),
        Box::new(ast::Expression::Identifier(String::from("c")))
    ));
    assert!(parser.parse("[f g, a + [], Option with 4]").unwrap() == ast::Expression::List(vec![
        ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::Identifier(String::from("g")))
        ),
        ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
                Box::new(ast::Expression::Identifier(String::from("a")))
            )),
            Box::new(ast::Expression::List(vec![]))
        ),
        ast::Expression::TypeVariant(String::from("Option"), Box::new(ast::Expression::IntegerLiteral(4)))
    ]));
    assert!(parser.parse("func Some 4 g 6 \"hi\"").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::FuncApplication(
                    Box::new(ast::Expression::FuncApplication(
                        Box::new(ast::Expression::Identifier(String::from("func"))),
                        Box::new(ast::Expression::Identifier(String::from("Some"))),
                    )),
                    Box::new(ast::Expression::IntegerLiteral(4))
                )),
                Box::new(ast::Expression::Identifier(String::from("g")))
            )),
            Box::new(ast::Expression::IntegerLiteral(6))
        )),
        Box::new(ast::Expression::StringLiteral(String::from("hi")))
    ));
    assert!(parser.parse("func (Some with 4) (g 5) \"hi\"").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier(String::from("func"))),
                Box::new(ast::Expression::TypeVariant(String::from("Some"), Box::new(
                    ast::Expression::IntegerLiteral(4)
                )))
            )),
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier(String::from("g"))),
                Box::new(ast::Expression::IntegerLiteral(5))
            ))
        )),
        Box::new(ast::Expression::StringLiteral(String::from("hi")))
    ));
    // as far as parsing is concerned, this is syntactically valid
    assert!(parser.parse("4 g").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::IntegerLiteral(4)),
        Box::new(ast::Expression::Identifier(String::from("g")))
    ));
    assert!(parser.parse("f (//) 2").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::BuiltinOp(ast::Operation::FloorDiv))
        )),
        Box::new(ast::Expression::IntegerLiteral(2))
    ));
    assert!(parser.parse("f // 2").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::FloorDiv)),
            Box::new(ast::Expression::Identifier(String::from("f")))
        )),
        Box::new(ast::Expression::IntegerLiteral(2))
    ));
    // Err function application
    assert!(parser.parse("f78").unwrap() != ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier(String::from("f"))),
        Box::new(ast::Expression::IntegerLiteral(78))
    ));
    assert!(parser.parse("fg").unwrap() != ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier(String::from("f"))),
        Box::new(ast::Expression::Identifier(String::from("g")))
    ));
    assert!(parser.parse("f(8)").is_err());
    assert!(parser.parse("__f\"hi\" 5 2").is_err());
    assert!(parser.parse("(a + b + c)").is_err());
    assert!(parser.parse("a / b * c").is_err());
    assert!(parser.parse("a + (b - c - d)").is_err());
    assert!(parser.parse("\"hi\" / ").is_err());
    assert!(parser.parse("x + sum xs").is_err());
}

#[test]
fn test_parse_lambda_expr() {
    let parser = grammar::ExpressionParser::new();
    // Ok
    assert!(parser.parse("\\ x y -> 4").unwrap() == ast::Expression::Lambda(
        vec![String::from("x"), String::from("y")],
        Box::new(ast::Expression::IntegerLiteral(4))
    ));
    assert!(parser.parse("\\x -> (+) x 5").unwrap() == ast::Expression::Lambda(
        vec![String::from("x")],
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
                Box::new(ast::Expression::Identifier(String::from("x"))),
            )),
            Box::new(ast::Expression::IntegerLiteral(5))
        ))
    ));
    assert!(parser.parse("\\x y -> Option with (x, y)").unwrap() == ast::Expression::Lambda(
        vec![String::from("x"), String::from("y")],
        Box::new(ast::Expression::TypeVariant(
            String::from("Option"),
            Box::new(ast::Expression::Tuple(vec![
                ast::Expression::Identifier(String::from("x")),
                ast::Expression::Identifier(String::from("y"))
            ]))
        ))
    ));
    assert!(parser.parse("\\x->f x").unwrap() == ast::Expression::Lambda(
        vec![String::from("x")],
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::Identifier(String::from("x")))
        ))
    ));
    // Err
    assert!(parser.parse("\\").is_err());
    assert!(parser.parse("\\ ->").is_err());
    assert!(parser.parse("\\ -> 5").is_err());
    assert!(parser.parse("\\ x (y) -> 5").is_err());
    assert!(parser.parse("x -> 5").is_err());
    assert!(parser.parse("\\string -> 5").is_err());
}

#[test]
fn test_parse_match() {
    let parser = grammar::ExpressionParser::new();
    // Ok
    assert!(parser.parse("match a { _ => 3 }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::Identifier(String::from("a"))),
        vec![(ast::Pattern::Wildcard, ast::Expression::IntegerLiteral(3))]
    ));
    assert!(parser.parse("match f g {
        7 | -8.6 => 5 - 6,
        x :: xs if x == 4 => x *\"hi\",
        _ => 4,
    }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::Identifier(String::from("g")))
        )),
        vec![
            (
                ast::Pattern::Union(vec![
                    ast::Pattern::IntegerLiteral(7),
                    ast::Pattern::FloatLiteral(OrderedFloat(-8.6))
                ]),
                ast::Expression::FuncApplication(
                    Box::new(ast::Expression::FuncApplication(
                        Box::new(ast::Expression::BuiltinOp(ast::Operation::Subtract)),
                        Box::new(ast::Expression::IntegerLiteral(5))
                    )),
                    Box::new(ast::Expression::IntegerLiteral(6))
                )
            ),
            (
                ast::Pattern::Guarded(
                    Box::new(ast::Pattern::ListConstruction(String::from("x"), String::from("xs"))),
                    ast::Expression::FuncApplication(
                        Box::new(ast::Expression::FuncApplication(
                            Box::new(ast::Expression::BuiltinOp(ast::Operation::Eq)),
                            Box::new(ast::Expression::Identifier(String::from("x")))
                        )),
                        Box::new(ast::Expression::IntegerLiteral(4))
                    )
                ),
                ast::Expression::FuncApplication(
                    Box::new(ast::Expression::FuncApplication(
                        Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
                        Box::new(ast::Expression::Identifier(String::from("x")))
                    )),
                    Box::new(ast::Expression::StringLiteral(String::from("hi")))
                )
            ),
            (
                ast::Pattern::Wildcard,
                ast::Expression::IntegerLiteral(4)
            )
        ]
    ));
    assert!(parser.parse("match f g {
        x => y,
    }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::Identifier(String::from("g")))
        )),
        vec![(
            ast::Pattern::Identifier(String::from("x")),
            ast::Expression::Identifier(String::from("y"))
        )]
    ));
    assert!(parser.parse("match f g {
        x => y
    }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::Identifier(String::from("g")))
        )),
        vec![(
            ast::Pattern::Identifier(String::from("x")),
            ast::Expression::Identifier(String::from("y"))
        )]
    ));

    assert!(parser.parse("match 4 {
        3 => 6,
        _ => 7
    }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::IntegerLiteral(4)),
        vec![(
            ast::Pattern::IntegerLiteral(3),
            ast::Expression::IntegerLiteral(6),
        ), (
            ast::Pattern::Wildcard,
            ast::Expression::IntegerLiteral(7)
        )]
    ));
    assert!(parser.parse("match 4 {
        3 => 6,
        _ => 7,
    }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::IntegerLiteral(4)),
        vec![(
            ast::Pattern::IntegerLiteral(3),
            ast::Expression::IntegerLiteral(6),
        ), (
            ast::Pattern::Wildcard,
            ast::Expression::IntegerLiteral(7)
        )]
    ));
    assert!(parser.parse("match (f g) { _ => x }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::Identifier(String::from("g")))
        )),
        vec![(ast::Pattern::Wildcard, ast::Expression::Identifier(String::from("x")))]
    ));
    // Err
    assert!(parser.parse("match 4 {
        3 => 4
        5 => 6
    }").is_err());
    assert!(parser.parse("match 4 {
        3 => 4 => 5
    }").is_err());
    assert!(parser.parse("match 4 {
        3 =>
    }").is_err());
    assert!(parser.parse("match 4 {
        3 => 6,
        _ = > 7
    }").is_err());
    assert!(parser.parse("match 4 {
        3 => 6,
        _ = > 7
    }").is_err());
    assert!(parser.parse("match 4 { }").is_err());
    assert!(parser.parse("match (f g)").is_err());
    assert!(parser.parse("match f g").is_err());
    assert!(parser.parse("match(f g) { _ => x }").is_err());
    assert!(parser.parse("match5 { _ => x }").is_err());
    assert!(parser.parse("match-6.7 { _ => x }").is_err());
}

// Type Expressions

#[test]
fn test_parse_type_lit_type_var() {
    let parser = grammar::TypeExpressionParser::new();
    // Ok literal type
    assert!(parser.parse("int").unwrap() == ast::TypeExpression::IntType);
    assert!(parser.parse("float").unwrap() == ast::TypeExpression::FloatType);
    assert!(parser.parse("string").unwrap() == ast::TypeExpression::StringType);
    assert!(parser.parse("iNT").unwrap() == ast::TypeExpression::DeclaredType(String::from("iNT"), vec![]));
    assert!(parser.parse("_x").unwrap() == ast::TypeExpression::DeclaredType(String::from("_x"), vec![]));
    assert!(parser.parse("flot").unwrap() == ast::TypeExpression::DeclaredType(String::from("flot"), vec![]));
    assert!(parser.parse("(int)").unwrap() == ast::TypeExpression::IntType);
    assert!(parser.parse("(xello)").unwrap() == ast::TypeExpression::DeclaredType(String::from("xello"), vec![]));
    // Err literal type
    assert!(parser.parse("Un[").is_err());
    assert!(parser.parse("()").is_err());
    // Ok type variable
    assert!(parser.parse("'a").unwrap() == ast::TypeExpression::UniversalType(String::from("a")));
    assert!(parser.parse("'_yusdf").unwrap() == ast::TypeExpression::UniversalType(String::from("_yusdf")));
    assert!(parser.parse("'aAbABBB").unwrap() == ast::TypeExpression::UniversalType(String::from("aAbABBB")));
    assert!(parser.parse("'v1").unwrap() == ast::TypeExpression::UniversalType(String::from("v1")));
    assert!(parser.parse("'Type").unwrap() == ast::TypeExpression::UniversalType(String::from("Type")));
    assert!(parser.parse("'___Type").unwrap() == ast::TypeExpression::UniversalType(String::from("___Type")));
    // Err type variable
    assert!(parser.parse("''").is_err());
    assert!(parser.parse("'hello'").is_err());
    assert!(parser.parse("'950abc").is_err());
    assert!(parser.parse("8").is_err());
    assert!(parser.parse("\"hello\"").is_err());
    assert!(parser.parse("x:int").is_err());
    assert!(parser.parse("'aபாதை").is_err());
}

#[test]
fn test_parse_list_tuple_type() {
    let parser = grammar::TypeExpressionParser::new();
    // Ok list type
    assert!(parser.parse("[int]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::IntType
    )));
    assert!(parser.parse("[Option]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::DeclaredType(String::from("Option"), vec![])
    )));
    assert!(parser.parse("['a]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::UniversalType(String::from("a"))
    )));
    assert!(parser.parse("[(int)]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::IntType
    )));
    assert!(parser.parse("[(int, Option)]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::TupleType(vec![
            ast::TypeExpression::IntType,
            ast::TypeExpression::DeclaredType(String::from("Option"), vec![])
        ])
    )));
    assert!(parser.parse("[[int]]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::ListType(Box::new(
            ast::TypeExpression::IntType
        ))
    )));
    assert!(parser.parse("[Option int]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::DeclaredType(String::from("Option"), vec![
            ast::TypeExpression::IntType
        ])
    )));
    // Err list type
    assert!(parser.parse("[int").is_err());
    assert!(parser.parse("[int, string]").is_err());
    assert!(parser.parse("hel]o").is_err());
    // Ok tuple type
    assert!(parser.parse("('a, )").unwrap() == ast::TypeExpression::TupleType(vec![
        ast::TypeExpression::UniversalType(String::from("a"))
    ]));
    assert!(parser.parse("(int, string, float)").unwrap() == ast::TypeExpression::TupleType(vec![
        ast::TypeExpression::IntType,
        ast::TypeExpression::StringType,
        ast::TypeExpression::FloatType
    ]));
    assert!(parser.parse("(Option int float, int, Option, string)").unwrap() == ast::TypeExpression::TupleType(vec![
        ast::TypeExpression::DeclaredType(String::from("Option"), vec![
            ast::TypeExpression::IntType,
            ast::TypeExpression::FloatType
        ]),
        ast::TypeExpression::IntType,
        ast::TypeExpression::DeclaredType(String::from("Option"), vec![]),
        ast::TypeExpression::StringType
    ]));
    // Err tuple type
    assert!(parser.parse("()").is_err());
    assert!(parser.parse("int, string)").is_err());
    assert!(parser.parse("int, string, 'a").is_err());
    assert!(parser.parse("hi(, there").is_err());
    assert!(parser.parse("(xello, int, stri)ng, )").is_err());
}

#[test]
fn test_parse_function_type() {
    let parser = grammar::TypeExpressionParser::new();
    // Ok function type
    assert!(parser.parse("int -> float").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::IntType),
        Box::new(ast::TypeExpression::FloatType)
    ));
    assert!(parser.parse("(int -> float)").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::IntType),
        Box::new(ast::TypeExpression::FloatType)
    ));
    assert!(parser.parse("(int->float->        string ->Option)").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::FunctionType(
                Box::new(ast::TypeExpression::IntType),
                Box::new(ast::TypeExpression::FloatType)
            )),
            Box::new(ast::TypeExpression::StringType)
        )),
        Box::new(ast::TypeExpression::DeclaredType(String::from("Option"), vec![]))
    ));
    assert!(parser.parse("'a -> 'b -> (int)").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::UniversalType(String::from("a"))),
            Box::new(ast::TypeExpression::UniversalType(String::from("b")))
        )),
        Box::new(ast::TypeExpression::IntType)
    ));
    assert!(parser.parse("string -> Option int -> float").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::StringType),
            Box::new(ast::TypeExpression::DeclaredType(String::from("Option"), vec![ast::TypeExpression::IntType]))
        )),
        Box::new(ast::TypeExpression::FloatType)
    ));
    assert!(parser.parse("'a -> ('a -> int) -> 'a").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::UniversalType(String::from("a"))),
            Box::new(ast::TypeExpression::FunctionType(
                Box::new(ast::TypeExpression::UniversalType(String::from("a"))),
                Box::new(ast::TypeExpression::IntType)
            ))
        )),
        Box::new(ast::TypeExpression::UniversalType(String::from("a")))
    ));
    // Err function type
    assert!(parser.parse("int ->").is_err());
    assert!(parser.parse("-> float").is_err());
    assert!(parser.parse("(int -> float").is_err());
    assert!(parser.parse("(int - > float)").is_err());
    assert!(parser.parse("(int -> 4)").is_err());
    assert!(parser.parse("'a int -> int").is_err());
    assert!(parser.parse("x: int -> y: float -> string").is_err());
}

#[test]
fn test_parse_declared_type() {
    let parser = grammar::TypeExpressionParser::new();
    // Ok declared type
    assert!(parser.parse("bool").unwrap() == ast::TypeExpression::DeclaredType(String::from("bool"), vec![]));
    assert!(parser.parse("Option int").unwrap() == ast::TypeExpression::DeclaredType(
        String::from("Option"), vec![ast::TypeExpression::IntType]
    ));
    assert!(parser.parse("Tree (Tree) float").unwrap() == ast::TypeExpression::DeclaredType(
        String::from("Tree"),
        vec![
            ast::TypeExpression::DeclaredType(String::from("Tree"), vec![]),
            ast::TypeExpression::FloatType
        ]
    ));
    assert!(parser.parse("Tree (Tree float)").unwrap() == ast::TypeExpression::DeclaredType(
        String::from("Tree"),
        vec![ast::TypeExpression::DeclaredType(String::from("Tree"), vec![
                ast::TypeExpression::FloatType
        ])]
    ));
    // Err declared type
    assert!(parser.parse("Option \"hi\"").is_err());
    assert!(parser.parse("Tree Tree float").is_err());
    assert!(parser.parse("(Tree) float").is_err());
    assert!(parser.parse("(Tree) 'a").is_err());
    assert!(parser.parse("bool'a").is_err());
    assert!(parser.parse("(yello").is_err());
    assert!(parser.parse("bool [int,]").is_err());
    assert!(parser.parse("(yello").is_err());
    assert!(parser.parse("X with int").is_err());
}

// Patterns

#[test]
fn test_parse_atomic_pattern() {
    let parser = grammar::PatternParser::new();
    // Ok atomic
    assert!(parser.parse("_").unwrap() == ast::Pattern::Wildcard);
    assert!(parser.parse("__").unwrap() == ast::Pattern::Identifier(String::from("__")));
    assert!(parser.parse("-6789").unwrap() == ast::Pattern::IntegerLiteral(-6789));
    assert!(parser.parse("9.8e3").unwrap() == ast::Pattern::FloatLiteral(OrderedFloat(9800.0)));
    assert!(parser.parse("\"helloⓓⓕ\"").unwrap() == ast::Pattern::StringLiteral(String::from("helloⓓⓕ")));
    assert!(parser.parse("x").unwrap() == ast::Pattern::Identifier(String::from("x")));
    assert!(parser.parse("__o98").unwrap() == ast::Pattern::Identifier(String::from("__o98")));
    assert!(parser.parse("Some with x").unwrap() == ast::Pattern::TypeVariant(
        String::from("Some"), Box::new(ast::Pattern::Identifier(String::from("x")))
    ));
    assert!(parser.parse("Option with (Tree, 4)").unwrap() == ast::Pattern::TypeVariant(
        String::from("Option"), Box::new(ast::Pattern::Tuple(vec![
            ast::Pattern::Identifier(String::from("Tree")),
            ast::Pattern::IntegerLiteral(4)
        ]))
    ));
    assert!(parser.parse("Some with 4").unwrap() == ast::Pattern::TypeVariant(
        String::from("Some"), Box::new(ast::Pattern::IntegerLiteral(4))
    ));
    assert!(parser.parse("[ ]").unwrap() == ast::Pattern::EmptyList);
    assert!(parser.parse("bool with (List with [_, x :: xs])").unwrap() == ast::Pattern::TypeVariant(
        String::from("bool"), Box::new(ast::Pattern::TypeVariant(
            String::from("List"), Box::new(ast::Pattern::List(vec![
                ast::Pattern::Wildcard,
                ast::Pattern::ListConstruction(String::from("x"), String::from("xs"))
            ]))
        ))
    ));
    assert!(parser.parse("Integer with (4)").unwrap() == ast::Pattern::TypeVariant(
        String::from("Integer"), Box::new(ast::Pattern::IntegerLiteral(4))
    ));
    // Err atomic
    assert!(parser.parse("=>").is_err());
    assert!(parser.parse("ⓓⓕ").is_err());
    assert!(parser.parse("- 5").is_err());
    assert!(parser.parse("__ => _").is_err());
    assert!(parser.parse("98x").is_err());
    assert!(parser.parse("Some with int").is_err());
    assert!(parser.parse("Option x").is_err());
    assert!(parser.parse("Thingy with ([x, y])").is_err());
}

#[test]
fn test_parse_compound_pattern() {
    let parser = grammar::PatternParser::new();
    // Ok compound
    assert!(parser.parse("[4, _, _, []]").unwrap() == ast::Pattern::List(vec![
        ast::Pattern::IntegerLiteral(4),
        ast::Pattern::Wildcard,
        ast::Pattern::Wildcard,
        ast::Pattern::EmptyList
    ]));
    assert!(parser.parse("Integer with (Option with y, x :: xs)").unwrap() == ast::Pattern::TypeVariant(
        String::from("Integer"), Box::new(ast::Pattern::Tuple(vec![
            ast::Pattern::TypeVariant(String::from("Option"), Box::new(ast::Pattern::Identifier(String::from("y")))),
            ast::Pattern::ListConstruction(String::from("x"), String::from("xs"))
        ]))
    ));
    assert!(parser.parse("Float with (5.7,)").unwrap() == ast::Pattern::TypeVariant(
        String::from("Float"),
        Box::new(ast::Pattern::Tuple(vec![
            ast::Pattern::FloatLiteral(OrderedFloat(5.7))
        ]))
    ));
    assert!(parser.parse("(_, _, v)").unwrap() == ast::Pattern::Tuple(vec![
        ast::Pattern::Wildcard,
        ast::Pattern::Wildcard,
        ast::Pattern::Identifier(String::from("v"))
    ]));
    assert!(parser.parse("[4, -5.6, _]").unwrap() == ast::Pattern::List(vec![
        ast::Pattern::IntegerLiteral(4),
        ast::Pattern::FloatLiteral(OrderedFloat(-5.6)),
        ast::Pattern::Wildcard
    ]));
    assert!(parser.parse("(7, )").unwrap() == ast::Pattern::Tuple(vec![ast::Pattern::IntegerLiteral(7)]));
    // Err compound
    assert!(parser.parse("(_, 4").is_err());
    assert!(parser.parse("( )").is_err());
    assert!(parser.parse("[x, y").is_err());
    assert!(parser.parse("[4,]").is_err());
    assert!(parser.parse("(x)with y").is_err());
    assert!(parser.parse("with 4").is_err());
    assert!(parser.parse("[[Some with x]]").is_err());
    assert!(parser.parse("(x, _, (4, 5))").is_err());
    assert!(parser.parse("([4, -5.6, _], (7, ), x, )").is_err());
}

#[test]
fn test_parse_complex_pattern() {
    let parser = grammar::PatternParser::new();
    // Ok Pattern union
    assert!(parser.parse("x | y").unwrap() == ast::Pattern::Union(vec![
        ast::Pattern::Identifier(String::from("x")),
        ast::Pattern::Identifier(String::from("y"))
    ]));
    assert!(parser.parse("(4) | 5.0 | \"hello\"").unwrap() == ast::Pattern::Union(vec![
        ast::Pattern::IntegerLiteral(4),
        ast::Pattern::FloatLiteral(OrderedFloat(5.0)),
        ast::Pattern::StringLiteral(String::from("hello"))
    ]));
    assert!(parser.parse("x|(y)").unwrap() == ast::Pattern::Union(vec![
        ast::Pattern::Identifier(String::from("x")),
        ast::Pattern::Identifier(String::from("y"))
    ]));
    // Err pattern union
    assert!(parser.parse("[a, b] | 4").is_err());
    assert!(parser.parse("b|(x,)").is_err());
    assert!(parser.parse("4 |").is_err());
    assert!(parser.parse("|").is_err());
    assert!(parser.parse("4 | 5 | ").is_err());
    assert!(parser.parse("| 6.0 | 7").is_err());
    assert!(parser.parse("([x, y]) | (4)").is_err());
    // Ok Pattern complement
    assert!(parser.parse("~4").unwrap() == ast::Pattern::Complement(
        Box::new(ast::Pattern::IntegerLiteral(4))
    ));
    assert!(parser.parse("~[_, 4]").unwrap() == ast::Pattern::Complement(
        Box::new(ast::Pattern::List(vec![
            ast::Pattern::Wildcard,
            ast::Pattern::IntegerLiteral(4)
        ]))
    ));
    assert!(parser.parse("~Option with 4").unwrap() == ast::Pattern::Complement(
        Box::new(ast::Pattern::TypeVariant(
            String::from("Option"), Box::new(ast::Pattern::IntegerLiteral(4))
        ))
    ));
    assert!(parser.parse("~(Option with (2, 3, x, ))").unwrap() == ast::Pattern::Complement(
        Box::new(ast::Pattern::TypeVariant(
            String::from("Option"), Box::new(ast::Pattern::Tuple(vec![
                ast::Pattern::IntegerLiteral(2),
                ast::Pattern::IntegerLiteral(3),
                ast::Pattern::Identifier(String::from("x"))
            ]))
        ))
    ));
    // Err Pattern complement
    assert!(parser.parse("!").is_err());
    assert!(parser.parse("~").is_err());
    assert!(parser.parse("~~4").is_err());
    assert!(parser.parse("~(~4)").is_err());
    assert!(parser.parse("4~").is_err());
    assert!(parser.parse("5 | ~6").is_err());
    assert!(parser.parse("~ 5 | 4").is_err());
    // Ok guarded pattern
    assert!(parser.parse("x | y if true").unwrap() == ast::Pattern::Guarded(
        Box::new(ast::Pattern::Union(vec![
            ast::Pattern::Identifier(String::from("x")),
            ast::Pattern::Identifier(String::from("y"))
        ])),
        ast::Expression::Identifier(String::from("true"))
    ));
    assert!(parser.parse("x :: xs if f x").unwrap() == ast::Pattern::Guarded(
        Box::new(ast::Pattern::ListConstruction(String::from("x"), String::from("xs"))),
        ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier(String::from("f"))),
            Box::new(ast::Expression::Identifier(String::from("x")))
        )
    ));
    // Err guarded pattern
    assert!(parser.parse("if").is_err());
    assert!(parser.parse("x | 4if true").is_err());
    assert!(parser.parse("x | yif true").is_err());
    assert!(parser.parse("4 | 5 if(f g)").is_err());
    assert!(parser.parse("[a, b] if").is_err());
    assert!(parser.parse("(f g)").is_err());
}

// Statements

#[test]
fn test_parse_let() {
    let parser = grammar::StatementParser::new();
    // Ok untyped
    assert!(parser.parse("let x = 4;").unwrap() == ast::Statement::UntypedLet(
        vec![String::from("x")],
        ast::Expression::IntegerLiteral(4)
    ));
    assert!(parser.parse("let x y = (\\ x -> x) y;").unwrap() == ast::Statement::UntypedLet(
        vec![String::from("x"), String::from("y")],
        ast::Expression::FuncApplication(
            Box::new(ast::Expression::Lambda(vec![String::from("x")], Box::new(ast::Expression::Identifier(String::from("x"))))),
            Box::new(ast::Expression::Identifier(String::from("y")))
        )
    ));
    assert!(parser.parse("let plus_4 x = (+) 4;").unwrap() == ast::Statement::UntypedLet(
        vec![String::from("plus_4"), String::from("x")],
        ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
            Box::new(ast::Expression::IntegerLiteral(4))
        ))
    );
    assert!(parser.parse("let j x y z w = match plus_4 x {
        4 | 5 => [1, 2, y],
        w => 2,
        x :: xs => x // 4,
        _ => 8.6 * z
    };").unwrap() == ast::Statement::UntypedLet(
        vec![String::from("j"), String::from("x"), String::from("y"), String::from("z"), String::from("w")],
        ast::Expression::MatchConstruct(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier(String::from("plus_4"))),
                Box::new(ast::Expression::Identifier(String::from("x")))
            )),
            vec![(
                ast::Pattern::Union(vec![
                    ast::Pattern::IntegerLiteral(4),
                    ast::Pattern::IntegerLiteral(5)
                ]),
                ast::Expression::List(vec![
                    ast::Expression::IntegerLiteral(1),
                    ast::Expression::IntegerLiteral(2),
                    ast::Expression::Identifier(String::from("y"))
                ])
            ), (
                ast::Pattern::Identifier(String::from("w")),
                ast::Expression::IntegerLiteral(2)
            ), (
                ast::Pattern::ListConstruction(String::from("x"), String::from("xs")),
                ast::Expression::FuncApplication(
                    Box::new(ast::Expression::FuncApplication(
                        Box::new(ast::Expression::BuiltinOp(ast::Operation::FloorDiv)),
                        Box::new(ast::Expression::Identifier(String::from("x")))
                    )),
                    Box::new(ast::Expression::IntegerLiteral(4))
                )
            ), (
                ast::Pattern::Wildcard,
                ast::Expression::FuncApplication(
                    Box::new(ast::Expression::FuncApplication(
                        Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
                        Box::new(ast::Expression::FloatLiteral(OrderedFloat(8.6)))
                    )),
                    Box::new(ast::Expression::Identifier(String::from("z")))
                )
            )]
        )
    ));
    assert!(parser.parse("let y = { 5 };").unwrap() == ast::Statement::UntypedLet(
        vec![String::from("y")],
        ast::Expression::Block(vec![], Box::new(ast::Expression::IntegerLiteral(5)))
    ));
    assert!(parser.parse("let doublesum lst = {
        let sum lst = match lst {
            [] => 0,
            x::xs => x + (sum xs)
        };
        (*) 2 (sum lst)
    };").unwrap() == ast::Statement::UntypedLet(
        vec![String::from("doublesum"), String::from("lst")],
        ast::Expression::Block(
            vec![ast::Statement::UntypedLet(
                vec![String::from("sum"), String::from("lst")],
                ast::Expression::MatchConstruct(
                    Box::new(ast::Expression::Identifier(String::from("lst"))),
                    vec![(
                        ast::Pattern::EmptyList,
                        ast::Expression::IntegerLiteral(0)
                    ), (
                        ast::Pattern::ListConstruction(String::from("x"), String::from("xs")),
                        ast::Expression::FuncApplication(
                            Box::new(ast::Expression::FuncApplication(
                                Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
                                Box::new(ast::Expression::Identifier(String::from("x")))
                            )),
                            Box::new(ast::Expression::FuncApplication(
                                Box::new(ast::Expression::Identifier(String::from("sum"))),
                                Box::new(ast::Expression::Identifier(String::from("xs")))
                            ))
                        )
                    )]
                )
            )],
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::FuncApplication(
                    Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
                    Box::new(ast::Expression::IntegerLiteral(2))
                )),
                Box::new(ast::Expression::FuncApplication(
                    Box::new(ast::Expression::Identifier(String::from("sum"))),
                    Box::new(ast::Expression::Identifier(String::from("lst")))
                ))
            ))
        )
    ));
    assert!(parser.parse("let Main = print (f g);").unwrap() == ast::Statement::UntypedLet(
        vec![String::from("Main")], ast::Expression::FuncApplication(
            Box::new(ast::Expression::Print),
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier(String::from("f"))),
                Box::new(ast::Expression::Identifier(String::from("g")))
            ))
        )
    ));
    // Err untyped
    assert!(parser.parse("let x = 4").is_err());
    assert!(parser.parse("let x = ;").is_err());
    assert!(parser.parse("let x -> y = 4").is_err());
    assert!(parser.parse("let  = 4;").is_err());
    assert!(parser.parse("let func (x) y = x + y").is_err());
    // Ok typed
    assert!(parser.parse("let z: 'a = \"hi\";").unwrap() == ast::Statement::TypedLet(
        String::from("z"), ast::TypeExpression::UniversalType(String::from("a")), vec![], ast::Expression::StringLiteral(String::from("hi"))
    ));
    assert!(parser.parse("let y: float = 5.68;").unwrap() == ast::Statement::TypedLet(
        String::from("y"), ast::TypeExpression::FloatType, vec![], ast::Expression::FloatLiteral(OrderedFloat(5.68))
    ));
    assert!(parser.parse("let x: (int) = 4;").unwrap() == ast::Statement::TypedLet(
        String::from("x"), ast::TypeExpression::IntType, vec![], ast::Expression::IntegerLiteral(4)
    ));
    assert!(parser.parse("let x: Option int = Some with 4;").unwrap() == ast::Statement::TypedLet(
        String::from("x"), ast::TypeExpression::DeclaredType(String::from("Option"), vec![ast::TypeExpression::IntType]),
        vec![], ast::Expression::TypeVariant(String::from("Some"), Box::new(ast::Expression::IntegerLiteral(4)))
    ));
    // unfortunate, but it's not worth editing the parser to prevent this
    assert!(parser.parse("let func(x:int)->(y:int)->int=4;").unwrap() == ast::Statement::TypedLet(
        String::from("func"), ast::TypeExpression::IntType,
        vec![
            (String::from("x"), ast::TypeExpression::IntType),
            (String::from("y"), ast::TypeExpression::IntType)
        ],
        ast::Expression::IntegerLiteral(4)
    ));
    assert!(parser.parse("let func (x: bool) -> int -> Option 'a = \\ y -> (+) x y;").unwrap() == ast::Statement::TypedLet(
        String::from("func"),
        ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::IntType),
            Box::new(ast::TypeExpression::DeclaredType(String::from("Option"), vec![ast::TypeExpression::UniversalType(String::from("a"))]))
        ),
        vec![(String::from("x"), ast::TypeExpression::DeclaredType(String::from("bool"), vec![]))],
        ast::Expression::Lambda(
            vec![String::from("y")],
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::FuncApplication(
                    Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
                    Box::new(ast::Expression::Identifier(String::from("x")))
                )),
                Box::new(ast::Expression::Identifier(String::from("y")))
            ))
        )
    ));
    assert!(parser.parse("let func: int -> bool = \\ x -> (<=) 3 x;").unwrap() == ast::Statement::TypedLet(
        String::from("func"), ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::IntType),
            Box::new(ast::TypeExpression::DeclaredType(String::from("bool"), vec![]))
        ),
        vec![], ast::Expression::Lambda(vec![String::from("x")], Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::BuiltinOp(ast::Operation::Leq)),
                Box::new(ast::Expression::IntegerLiteral(3))
            )),
            Box::new(ast::Expression::Identifier(String::from("x")))
        )))
    ));
    // Err typed
    assert!(parser.parse("let x y: int -> int = y").is_err());
    assert!(parser.parse("let x = int;").is_err());
    assert!(parser.parse("let x: int;").is_err());
    assert!(parser.parse("let func: 'a -> 'a;").is_err());
    assert!(parser.parse("let x: 4 = 5;").is_err());
    assert!(parser.parse("let func (x: 8) -> (y: 9) = 8 + 9;").is_err());
    assert!(parser.parse("let func (x: int) - > int = 4 + x;").is_err());
    assert!(parser.parse("let func(x: int) -> (y: int) = 4;").is_err());
}

#[test]
fn test_parse_type_decl() {
    let parser = grammar::StatementParser::new();
    // Ok
    assert!(parser.parse("type bool = false | true;").unwrap() == ast::Statement::TypeDeclaration(
        String::from("bool"), vec![], vec![(String::from("false"), None), (String::from("true"), None)]
    ));
    assert!(parser.parse("type Option 'a = None | Some with 'a;").unwrap() == ast::Statement::TypeDeclaration(
        String::from("Option"), vec![String::from("a")], vec![
            (String::from("None"), None),
            (String::from("Some"), Some(ast::TypeExpression::UniversalType(String::from("a"))))
        ]
    ));
    assert!(parser.parse("type Thingy = Var1
        | Var2 with int
        | Var3 with string
        | Var4 with [int]
        |Var5 with (int, string, Thingy)
    ;").unwrap() == ast::Statement::TypeDeclaration(
        String::from("Thingy"), vec![], vec![
            (String::from("Var1"), None),
            (String::from("Var2"), Some(ast::TypeExpression::IntType)),
            (String::from("Var3"), Some(ast::TypeExpression::StringType)),
            (String::from("Var4"), Some(ast::TypeExpression::ListType(Box::new(ast::TypeExpression::IntType)))),
            (String::from("Var5"), Some(ast::TypeExpression::TupleType(vec![
                ast::TypeExpression::IntType,
                ast::TypeExpression::StringType,
                ast::TypeExpression::DeclaredType(String::from("Thingy"), vec![])
            ])))
        ]
    ));
    assert!(parser.parse("
    type binary_tree 'a = Leaf
        | Node with ('a, binary_tree 'a, binary_tree 'a);").unwrap() == ast::Statement::TypeDeclaration(
        String::from("binary_tree"), vec![String::from("a")], vec![
            (String::from("Leaf"), None),
            (String::from("Node"), Some(ast::TypeExpression::TupleType(vec![
                ast::TypeExpression::UniversalType(String::from("a")),
                ast::TypeExpression::DeclaredType(String::from("binary_tree"), vec![ast::TypeExpression::UniversalType(String::from("a"))]),
                ast::TypeExpression::DeclaredType(String::from("binary_tree"), vec![ast::TypeExpression::UniversalType(String::from("a"))])
            ])))
        ]
    ));
    assert!(parser.parse("type Onevar = Onevar;").unwrap() == ast::Statement::TypeDeclaration(
        String::from("Onevar"), vec![], vec![(String::from("Onevar"), None)]
    ));
    // Err
    assert!(parser.parse("type tvar = X with [int, string] | Y;").is_err());
    assert!(parser.parse("type Option 'a -> 'b = None | Some with ('a, 'b)").is_err());
    assert!(parser.parse("type Tree 'a = Leaf | Node + 'a;").is_err());
    assert!(parser.parse("typebool = false | true;").is_err());
    assert!(parser.parse("type bool 'a = match 'a { true => true, false => false };").is_err());
}

// Program

#[test]
fn test_parse_program() {
    let parser = grammar::ProgramParser::new();
    // Ok
    assert!(parser.parse("
        let f (g: 'a -> 'a) -> (x: 'a) -> 'a = g x;
        let double x = (*) 2 x;
        let z = f double 4; 
    ").unwrap() == vec![
        ast::Statement::TypedLet(
            String::from("f"), ast::TypeExpression::UniversalType(String::from("a")), vec![(
                String::from("g"), ast::TypeExpression::FunctionType(
                    Box::new(ast::TypeExpression::UniversalType(String::from("a"))),
                    Box::new(ast::TypeExpression::UniversalType(String::from("a")))
                )
            ), (
                String::from("x"), ast::TypeExpression::UniversalType(String::from("a"))
            )], ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier(String::from("g"))),
                Box::new(ast::Expression::Identifier(String::from("x")))
            )
        ),
        ast::Statement::UntypedLet(
            vec![String::from("double"), String::from("x")], ast::Expression::FuncApplication(
                Box::new(ast::Expression::FuncApplication(
                    Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
                    Box::new(ast::Expression::IntegerLiteral(2))
                )),
                Box::new(ast::Expression::Identifier(String::from("x")))
            )
        ),
        ast::Statement::UntypedLet(
            vec![String::from("z")], ast::Expression::FuncApplication(
                Box::new(ast::Expression::FuncApplication(
                    Box::new(ast::Expression::Identifier(String::from("f"))),
                    Box::new(ast::Expression::Identifier(String::from("double")))
                )),
                Box::new(ast::Expression::IntegerLiteral(4))
            )
        )
    ]);
    assert!(parser.parse("
        type Option 'a = None | Some with 'a;
        let print_optional (x: Option 'a) -> string = match x {
            None => print \"nothing!\",
            Some with x => print (\"something: \" + x) 
        };
    ").unwrap() == vec![
        ast::Statement::TypeDeclaration(String::from("Option"), vec![String::from("a")], vec![
            (String::from("None"), None), (String::from("Some"), Some(ast::TypeExpression::UniversalType(String::from("a"))))
        ]),
        ast::Statement::TypedLet(
            String::from("print_optional"), ast::TypeExpression::StringType,
            vec![(String::from("x"), ast::TypeExpression::DeclaredType(String::from("Option"), vec![ast::TypeExpression::UniversalType(String::from("a"))]))],
            ast::Expression::MatchConstruct(
                Box::new(ast::Expression::Identifier(String::from("x"))),
                vec![(
                    ast::Pattern::Identifier(String::from("None")),
                    ast::Expression::FuncApplication(
                        Box::new(ast::Expression::Print),
                        Box::new(ast::Expression::StringLiteral(String::from("nothing!")))
                    )
                ), (
                    ast::Pattern::TypeVariant(String::from("Some"), Box::new(ast::Pattern::Identifier(String::from("x")))),
                    ast::Expression::FuncApplication(
                        Box::new(ast::Expression::Print),
                        Box::new(ast::Expression::FuncApplication(
                            Box::new(ast::Expression::FuncApplication(
                                Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
                                Box::new(ast::Expression::StringLiteral(String::from("something: ")))
                            )),
                            Box::new(ast::Expression::Identifier(String::from("x")))
                        ))
                    )
                )]
            )
        )
    ]);
    assert!(parser.parse("
        type OptionalTuple 'a 'b = None | Some with ('a, 'b);
        let f (x: 'a) -> (y: 'b) -> OptionalTuple 'a 'b = Some with (x, y);
    ").unwrap() == vec![
        ast::Statement::TypeDeclaration(String::from("OptionalTuple"), vec![String::from("a"), String::from("b")], vec![
            (String::from("None"), None),
            (String::from("Some"), Some(ast::TypeExpression::TupleType(vec![
                ast::TypeExpression::UniversalType(String::from("a")),
                ast::TypeExpression::UniversalType(String::from("b"))
            ])))
        ]),
        ast::Statement::TypedLet(String::from("f"), ast::TypeExpression::DeclaredType(String::from("OptionalTuple"), vec![
            ast::TypeExpression::UniversalType(String::from("a")),
            ast::TypeExpression::UniversalType(String::from("b"))
        ]), vec![
            (String::from("x"), ast::TypeExpression::UniversalType(String::from("a"))),
            (String::from("y"), ast::TypeExpression::UniversalType(String::from("b")))
        ],
        ast::Expression::TypeVariant(String::from("Some"), Box::new(ast::Expression::Tuple(vec![
            ast::Expression::Identifier(String::from("x")),
            ast::Expression::Identifier(String::from("y"))
        ])))
    )]);
    // Err
    assert!(parser.parse("let x = 4 let y = 5").is_err());
    assert!(parser.parse("let x = ylet z = 4").is_err());
}
