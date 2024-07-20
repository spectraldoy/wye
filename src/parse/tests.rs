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
        ast::Expression::Identifier("x"),
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
    assert!(parser.parse("x").unwrap() == ast::Expression::Identifier("x"));
    assert!(parser.parse("identif").unwrap() == ast::Expression::Identifier("identif"));
    assert!(parser.parse("hElO_").unwrap() == ast::Expression::Identifier("hElO_"));
    assert!(parser.parse("_a0001").unwrap() == ast::Expression::Identifier("_a0001"));
    assert!(parser.parse("Hello").unwrap() == ast::Expression::Identifier("Hello"));
    assert!(parser.parse("__Option").unwrap() == ast::Expression::Identifier("__Option"));
    assert!(parser.parse("Ty6_Var68__iant_").unwrap() == ast::Expression::Identifier("Ty6_Var68__iant_"));
    assert!(parser.parse("___01").unwrap() == ast::Expression::Identifier("___01"));
    assert!(parser.parse("___").unwrap() == ast::Expression::Identifier("___"));
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
    // Ok type variant
    assert!(parser.parse("Some with 4").unwrap() == ast::Expression::TypeVariant(
        "Some", Box::new(ast::Expression::IntegerLiteral(4))
    ));
    assert!(parser.parse("(Thing with 4)").unwrap() == ast::Expression::TypeVariant(
        "Thing", Box::new(ast::Expression::IntegerLiteral(4))
    ));
    assert!(parser.parse("
        Node with (
            (Node with (Leaf, Leaf, 4)),
            Leaf,
            7
        )").unwrap() == 
        ast::Expression::TypeVariant("Node", Box::new(ast::Expression::Tuple(vec![
            ast::Expression::TypeVariant("Node", Box::new(ast::Expression::Tuple(vec![
                ast::Expression::Identifier("Leaf"),
                ast::Expression::Identifier("Leaf"),
                ast::Expression::IntegerLiteral(4)
            ]))),
            ast::Expression::Identifier("Leaf"),
            ast::Expression::IntegerLiteral(7)
        ])))
    );
    assert!(parser.parse("Listy with [1, \"hell⏰\"]").unwrap() == ast::Expression::TypeVariant("Listy", Box::new(
        ast::Expression::List(vec![
            ast::Expression::IntegerLiteral(1),
            ast::Expression::StringLiteral(String::from("hell⏰"))
        ])
    )));
    assert!(parser.parse("Bruh with (-5.2)").unwrap() == ast::Expression::TypeVariant(
        "Bruh", Box::new(ast::Expression::FloatLiteral(OrderedFloat(-5.2)))
    ));
    assert!(parser.parse("Bruh with (-5.2,)").unwrap() == ast::Expression::TypeVariant(
        "Bruh",
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
        Box::new(ast::Expression::Identifier("f")),
        Box::new(ast::Expression::Identifier("g"))
    ));
    assert!(parser.parse("f -7.9").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier("f")),
        Box::new(ast::Expression::FloatLiteral(OrderedFloat(-7.9)))
    ));
    assert!(parser.parse("f 4 2").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::IntegerLiteral(4))
        )),
        Box::new(ast::Expression::IntegerLiteral(2))
    ));
    assert!(parser.parse("(g 5)").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier("g")),
        Box::new(ast::Expression::IntegerLiteral(5))
    ));
    assert!(parser.parse("(g 4 \"hi\" (f 2))").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier("g")),
                Box::new(ast::Expression::IntegerLiteral(4))
            )),
            Box::new(ast::Expression::StringLiteral(String::from("hi")))
        )),
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier("f")),
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
            Box::new(ast::Expression::Identifier("a"))
        )),
        Box::new(ast::Expression::Identifier("b"))
    ));
    assert!(parser.parse("a//(b *6)").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::FloorDiv)),
            Box::new(ast::Expression::Identifier("a"))
        )),
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
                Box::new(ast::Expression::Identifier("b"))
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
                    Box::new(ast::Expression::Identifier("a"))
                )),
                Box::new(ast::Expression::Identifier("b"))
            ))
        )),
        Box::new(ast::Expression::Identifier("c"))
    ));
    assert!(parser.parse("[f g, a + [], Option with 4]").unwrap() == ast::Expression::List(vec![
        ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::Identifier("g"))
        ),
        ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
                Box::new(ast::Expression::Identifier("a"))
            )),
            Box::new(ast::Expression::List(vec![]))
        ),
        ast::Expression::TypeVariant("Option", Box::new(ast::Expression::IntegerLiteral(4)))
    ]));
    assert!(parser.parse("func Some 4 g 6 \"hi\"").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::FuncApplication(
                    Box::new(ast::Expression::FuncApplication(
                        Box::new(ast::Expression::Identifier("func")),
                        Box::new(ast::Expression::Identifier("Some")),
                    )),
                    Box::new(ast::Expression::IntegerLiteral(4))
                )),
                Box::new(ast::Expression::Identifier("g"))
            )),
            Box::new(ast::Expression::IntegerLiteral(6))
        )),
        Box::new(ast::Expression::StringLiteral(String::from("hi")))
    ));
    assert!(parser.parse("func (Some with 4) (g 5) \"hi\"").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier("func")),
                Box::new(ast::Expression::TypeVariant("Some", Box::new(
                    ast::Expression::IntegerLiteral(4)
                )))
            )),
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::Identifier("g")),
                Box::new(ast::Expression::IntegerLiteral(5))
            ))
        )),
        Box::new(ast::Expression::StringLiteral(String::from("hi")))
    ));
    // as far as parsing is concerned, this is syntactically valid
    assert!(parser.parse("4 g").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::IntegerLiteral(4)),
        Box::new(ast::Expression::Identifier("g"))
    ));
    assert!(parser.parse("f (//) 2").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::BuiltinOp(ast::Operation::FloorDiv))
        )),
        Box::new(ast::Expression::IntegerLiteral(2))
    ));
    assert!(parser.parse("f // 2").unwrap() == ast::Expression::FuncApplication(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::BuiltinOp(ast::Operation::FloorDiv)),
            Box::new(ast::Expression::Identifier("f"))
        )),
        Box::new(ast::Expression::IntegerLiteral(2))
    ));
    // Err function application
    assert!(parser.parse("f78").unwrap() != ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier("f")),
        Box::new(ast::Expression::IntegerLiteral(78))
    ));
    assert!(parser.parse("fg").unwrap() != ast::Expression::FuncApplication(
        Box::new(ast::Expression::Identifier("f")),
        Box::new(ast::Expression::Identifier("g"))
    ));
    assert!(parser.parse("f(8)").is_err());
    assert!(parser.parse("__f\"hi\" 5 2").is_err());
    assert!(parser.parse("(a + b + c)").is_err());
    assert!(parser.parse("a / b * c").is_err());
    assert!(parser.parse("a + (b - c - d)").is_err());
    assert!(parser.parse("\"hi\" / ").is_err())
}

#[test]
fn test_parse_lambda_expr() {
    let parser = grammar::ExpressionParser::new();
    // Ok
    assert!(parser.parse("\\ x y -> 4").unwrap() == ast::Expression::Lambda(
        vec!["x", "y"],
        Box::new(ast::Expression::IntegerLiteral(4))
    ));
    assert!(parser.parse("\\x -> (+) x 5").unwrap() == ast::Expression::Lambda(
        vec!["x"],
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::FuncApplication(
                Box::new(ast::Expression::BuiltinOp(ast::Operation::Add)),
                Box::new(ast::Expression::Identifier("x")),
            )),
            Box::new(ast::Expression::IntegerLiteral(5))
        ))
    ));
    assert!(parser.parse("\\x y -> Option with (x, y)").unwrap() == ast::Expression::Lambda(
        vec!["x", "y"],
        Box::new(ast::Expression::TypeVariant(
            "Option",
            Box::new(ast::Expression::Tuple(vec![
                ast::Expression::Identifier("x"),
                ast::Expression::Identifier("y")
            ]))
        ))
    ));
    assert!(parser.parse("\\x->f x").unwrap() == ast::Expression::Lambda(
        vec!["x"],
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::Identifier("x"))
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
        Box::new(ast::Expression::Identifier("a")),
        vec![(ast::Pattern::Wildcard, ast::Expression::IntegerLiteral(3))]
    ));
    assert!(parser.parse("match f g {
        7 | -8.6 => 5 - 6,
        x :: xs if x == 4 => x *\"hi\",
        _ => 4,
    }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::Identifier("g"))
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
                    Box::new(ast::Pattern::ListConstruction("x", "xs")),
                    ast::Expression::FuncApplication(
                        Box::new(ast::Expression::FuncApplication(
                            Box::new(ast::Expression::BuiltinOp(ast::Operation::Eq)),
                            Box::new(ast::Expression::Identifier("x"))
                        )),
                        Box::new(ast::Expression::IntegerLiteral(4))
                    )
                ),
                ast::Expression::FuncApplication(
                    Box::new(ast::Expression::FuncApplication(
                        Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
                        Box::new(ast::Expression::Identifier("x"))
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
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::Identifier("g"))
        )),
        vec![(
            ast::Pattern::Identifier("x"),
            ast::Expression::Identifier("y")
        )]
    ));
    assert!(parser.parse("match f g {
        x => y
    }").unwrap() == ast::Expression::MatchConstruct(
        Box::new(ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::Identifier("g"))
        )),
        vec![(
            ast::Pattern::Identifier("x"),
            ast::Expression::Identifier("y")
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
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::Identifier("g"))
        )),
        vec![(ast::Pattern::Wildcard, ast::Expression::Identifier("x"))]
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
    assert!(parser.parse("iNT").unwrap() == ast::TypeExpression::DeclaredType("iNT", vec![]));
    assert!(parser.parse("_x").unwrap() == ast::TypeExpression::DeclaredType("_x", vec![]));
    assert!(parser.parse("flot").unwrap() == ast::TypeExpression::DeclaredType("flot", vec![]));
    assert!(parser.parse("(int)").unwrap() == ast::TypeExpression::IntType);
    assert!(parser.parse("(xello)").unwrap() == ast::TypeExpression::DeclaredType("xello", vec![]));
    // Err literal type
    assert!(parser.parse("Un[").is_err());
    assert!(parser.parse("()").is_err());
    // Ok type variable
    assert!(parser.parse("'a").unwrap() == ast::TypeExpression::TypeVariable("a"));
    assert!(parser.parse("'_yusdf").unwrap() == ast::TypeExpression::TypeVariable("_yusdf"));
    assert!(parser.parse("'aAbABBB").unwrap() == ast::TypeExpression::TypeVariable("aAbABBB"));
    assert!(parser.parse("'v1").unwrap() == ast::TypeExpression::TypeVariable("v1"));
    assert!(parser.parse("'Type").unwrap() == ast::TypeExpression::TypeVariable("Type"));
    assert!(parser.parse("'___Type").unwrap() == ast::TypeExpression::TypeVariable("___Type"));
    // Err type variable
    assert!(parser.parse("''").is_err());
    assert!(parser.parse("'hello'").is_err());
    assert!(parser.parse("'950abc").is_err());
    assert!(parser.parse("8").is_err());
    assert!(parser.parse("\"hello\"").is_err());
    assert!(parser.parse("x:int").is_err());
}

#[test]
fn test_parse_list_tuple_type() {
    let parser = grammar::TypeExpressionParser::new();
    // Ok list type
    assert!(parser.parse("[int]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::IntType
    )));
    assert!(parser.parse("[Option]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::DeclaredType("Option", vec![])
    )));
    assert!(parser.parse("['a]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::TypeVariable("a")
    )));
    assert!(parser.parse("[(int)]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::IntType
    )));
    assert!(parser.parse("[(int, Option)]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::TupleType(vec![
            ast::TypeExpression::IntType,
            ast::TypeExpression::DeclaredType("Option", vec![])
        ])
    )));
    assert!(parser.parse("[[int]]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::ListType(Box::new(
            ast::TypeExpression::IntType
        ))
    )));
    assert!(parser.parse("[Option int]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::DeclaredType("Option", vec![
            ast::TypeExpression::IntType
        ])
    )));
    // Err list type
    assert!(parser.parse("[int").is_err());
    assert!(parser.parse("[int, string]").is_err());
    assert!(parser.parse("hel]o").is_err());
    // Ok tuple type
    assert!(parser.parse("('a, )").unwrap() == ast::TypeExpression::TupleType(vec![
        ast::TypeExpression::TypeVariable("a")
    ]));
    assert!(parser.parse("(int, string, float)").unwrap() == ast::TypeExpression::TupleType(vec![
        ast::TypeExpression::IntType,
        ast::TypeExpression::StringType,
        ast::TypeExpression::FloatType
    ]));
    assert!(parser.parse("(Option int float, int, Option, string)").unwrap() == ast::TypeExpression::TupleType(vec![
        ast::TypeExpression::DeclaredType("Option", vec![
            ast::TypeExpression::IntType,
            ast::TypeExpression::FloatType
        ]),
        ast::TypeExpression::IntType,
        ast::TypeExpression::DeclaredType("Option", vec![]),
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
        Box::new(ast::TypeExpression::DeclaredType("Option", vec![]))
    ));
    assert!(parser.parse("'a -> 'b -> (int)").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::TypeVariable("a")),
            Box::new(ast::TypeExpression::TypeVariable("b"))
        )),
        Box::new(ast::TypeExpression::IntType)
    ));
    assert!(parser.parse("string -> Option int -> float").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::StringType),
            Box::new(ast::TypeExpression::DeclaredType("Option", vec![ast::TypeExpression::IntType]))
        )),
        Box::new(ast::TypeExpression::FloatType)
    ));
    assert!(parser.parse("'a -> ('a -> int) -> 'a").unwrap() == ast::TypeExpression::FunctionType(
        Box::new(ast::TypeExpression::FunctionType(
            Box::new(ast::TypeExpression::TypeVariable("a")),
            Box::new(ast::TypeExpression::FunctionType(
                Box::new(ast::TypeExpression::TypeVariable("a")),
                Box::new(ast::TypeExpression::IntType)
            ))
        )),
        Box::new(ast::TypeExpression::TypeVariable("a"))
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
    assert!(parser.parse("bool").unwrap() == ast::TypeExpression::DeclaredType("bool", vec![]));
    assert!(parser.parse("Option int").unwrap() == ast::TypeExpression::DeclaredType(
        "Option", vec![ast::TypeExpression::IntType]
    ));
    assert!(parser.parse("Tree (Tree) float").unwrap() == ast::TypeExpression::DeclaredType(
        "Tree",
        vec![
            ast::TypeExpression::DeclaredType("Tree", vec![]),
            ast::TypeExpression::FloatType
        ]
    ));
    assert!(parser.parse("Tree (Tree float)").unwrap() == ast::TypeExpression::DeclaredType(
        "Tree",
        vec![ast::TypeExpression::DeclaredType("Tree", vec![
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
    assert!(parser.parse("__").unwrap() == ast::Pattern::Identifier("__"));
    assert!(parser.parse("-6789").unwrap() == ast::Pattern::IntegerLiteral(-6789));
    assert!(parser.parse("9.8e3").unwrap() == ast::Pattern::FloatLiteral(OrderedFloat(9800.0)));
    assert!(parser.parse("\"helloⓓⓕ\"").unwrap() == ast::Pattern::StringLiteral(String::from("helloⓓⓕ")));
    assert!(parser.parse("x").unwrap() == ast::Pattern::Identifier("x"));
    assert!(parser.parse("__o98").unwrap() == ast::Pattern::Identifier("__o98"));
    assert!(parser.parse("Some with x").unwrap() == ast::Pattern::TypeVariant(
        "Some", Box::new(ast::Pattern::Identifier("x"))
    ));
    assert!(parser.parse("Option with (Tree, 4)").unwrap() == ast::Pattern::TypeVariant(
        "Option", Box::new(ast::Pattern::Tuple(vec![
            ast::Pattern::Identifier("Tree"),
            ast::Pattern::IntegerLiteral(4)
        ]))
    ));
    assert!(parser.parse("Some with 4").unwrap() == ast::Pattern::TypeVariant(
        "Some", Box::new(ast::Pattern::IntegerLiteral(4))
    ));
    assert!(parser.parse("[ ]").unwrap() == ast::Pattern::EmptyList);
    assert!(parser.parse("bool with (List with [_, x :: xs])").unwrap() == ast::Pattern::TypeVariant(
        "bool", Box::new(ast::Pattern::TypeVariant(
            "List", Box::new(ast::Pattern::List(vec![
                ast::Pattern::Wildcard,
                ast::Pattern::ListConstruction("x", "xs")
            ]))
        ))
    ));
    assert!(parser.parse("Integer with (4)").unwrap() == ast::Pattern::TypeVariant(
        "Integer", Box::new(ast::Pattern::IntegerLiteral(4))
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
        "Integer", Box::new(ast::Pattern::Tuple(vec![
            ast::Pattern::TypeVariant("Option", Box::new(ast::Pattern::Identifier("y"))),
            ast::Pattern::ListConstruction("x", "xs")
        ]))
    ));
    assert!(parser.parse("Float with (5.7,)").unwrap() == ast::Pattern::TypeVariant(
        "Float",
        Box::new(ast::Pattern::Tuple(vec![
            ast::Pattern::FloatLiteral(OrderedFloat(5.7))
        ]))
    ));
    assert!(parser.parse("(_, _, v)").unwrap() == ast::Pattern::Tuple(vec![
        ast::Pattern::Wildcard,
        ast::Pattern::Wildcard,
        ast::Pattern::Identifier("v")
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
        ast::Pattern::Identifier("x"),
        ast::Pattern::Identifier("y")
    ]));
    assert!(parser.parse("(4) | 5.0 | \"hello\"").unwrap() == ast::Pattern::Union(vec![
        ast::Pattern::IntegerLiteral(4),
        ast::Pattern::FloatLiteral(OrderedFloat(5.0)),
        ast::Pattern::StringLiteral(String::from("hello"))
    ]));
    assert!(parser.parse("x|(y)").unwrap() == ast::Pattern::Union(vec![
        ast::Pattern::Identifier("x"),
        ast::Pattern::Identifier("y")
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
            "Option", Box::new(ast::Pattern::IntegerLiteral(4))
        ))
    ));
    assert!(parser.parse("~(Option with (2, 3, x, ))").unwrap() == ast::Pattern::Complement(
        Box::new(ast::Pattern::TypeVariant(
            "Option", Box::new(ast::Pattern::Tuple(vec![
                ast::Pattern::IntegerLiteral(2),
                ast::Pattern::IntegerLiteral(3),
                ast::Pattern::Identifier("x")
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
            ast::Pattern::Identifier("x"),
            ast::Pattern::Identifier("y")
        ])),
        ast::Expression::Identifier("true")
    ));
    assert!(parser.parse("x :: xs if f x").unwrap() == ast::Pattern::Guarded(
        Box::new(ast::Pattern::ListConstruction("x", "xs")),
        ast::Expression::FuncApplication(
            Box::new(ast::Expression::Identifier("f")),
            Box::new(ast::Expression::Identifier("x"))
        )
    ));
    // Err guarded pattern
    assert!(parser.parse("if").is_err());
    assert!(parser.parse("x | 4if true").is_err());
    assert!(parser.parse("x | yif true").is_err());
    assert!(parser.parse("4 | 5 if(f g)").is_err());
    assert!(parser.parse("[a, b] if").is_err());
}

// Statements
