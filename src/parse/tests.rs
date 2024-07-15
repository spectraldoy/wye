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
        ast::Expression::Variable(String::from("x")),
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
fn test_parse_identifier() {
    let parser = grammar::ExpressionParser::new();
    // Ok identifier
    assert!(parser.parse("x").unwrap() == ast::Expression::Variable(String::from("x")));
    assert!(parser.parse("identif").unwrap() == ast::Expression::Variable(String::from("identif")));
    assert!(parser.parse("hElO_").unwrap() == ast::Expression::Variable(String::from("hElO_")));
    assert!(parser.parse("_a0001").unwrap() == ast::Expression::Variable(String::from("_a0001")));
    // Err identifier
    assert!(parser.parse("string").is_err());
    assert!(parser.parse("with").is_err());
    assert!(parser.parse("int").is_err());
    assert!(parser.parse("let").is_err());
    assert!(parser.parse("type").is_err());
    assert!(parser.parse("<").is_err());
    assert!(parser.parse("yel⏰o").is_err());
    assert!(parser.parse("___01").is_err());
    assert!(parser.parse("31232abcd").is_err());
}

#[test]
fn test_parse_type_variant() {
    let parser = grammar::ExpressionParser::new();
    // Ok type variant
    assert!(parser.parse("Hello").unwrap() == ast::Expression::TypeVariant(String::from("Hello"), None));
    assert!(parser.parse("__Option").unwrap() == ast::Expression::TypeVariant(String::from("__Option"), None));
    assert!(parser.parse("Ty6_Var68__iant_").unwrap() == ast::Expression::TypeVariant(String::from("Ty6_Var68__iant_"), None));
    assert!(parser.parse("Some with 4").unwrap() == ast::Expression::TypeVariant(
        String::from("Some"),
        Some(Box::new(ast::Expression::IntegerLiteral(4)))
    ));
    assert!(parser.parse("
        Node with (
            (Node with (Leaf, Leaf, 4)),
            Leaf,
            7
        )").unwrap() == 
        ast::Expression::TypeVariant(String::from("Node"), Some(Box::new(ast::Expression::Tuple(vec![
            ast::Expression::TypeVariant(String::from("Node"), Some(Box::new(ast::Expression::Tuple(vec![
                ast::Expression::TypeVariant(String::from("Leaf"), None),
                ast::Expression::TypeVariant(String::from("Leaf"), None),
                ast::Expression::IntegerLiteral(4)
            ])))),
            ast::Expression::TypeVariant(String::from("Leaf"), None),
            ast::Expression::IntegerLiteral(7)
        ]))))
    );
    assert!(parser.parse("Listy with [1, \"hell⏰\"]").unwrap() == ast::Expression::TypeVariant(String::from("Listy"), Some(Box::new(
        ast::Expression::List(vec![
            ast::Expression::IntegerLiteral(1),
            ast::Expression::StringLiteral(String::from("hell⏰"))
        ])
    ))));
    assert!(parser.parse("Bruh with (-5.2)").unwrap() == ast::Expression::TypeVariant(
        String::from("Bruh"),
        Some(Box::new(ast::Expression::FloatLiteral(OrderedFloat(-5.2))))
    ));
    // Err type variant
    assert!(parser.parse("Hel)lo").is_err());
    assert!(parser.parse("31232AAA").is_err());
    assert!(parser.parse("_Yel⏰o").is_err());
    assert!(parser.parse("___").is_err());
    assert!(parser.parse("Yup with [8, 78").is_err());
    assert!(parser.parse("Option int").is_err());
}

// TODO: test parse function application

#[test]
fn test_parse_type_expr() {
    let parser = grammar::TypeExpressionParser::new();
    // Ok literal type
    assert!(parser.parse("int").unwrap() == ast::TypeExpression::IntType);
    assert!(parser.parse("float").unwrap() == ast::TypeExpression::FloatType);
    assert!(parser.parse("string").unwrap() == ast::TypeExpression::StringType);
    // Err literal type
    assert!(parser.parse("iNT").is_err());
    assert!(parser.parse("_x").is_err());
    assert!(parser.parse("flot").is_err());
    // Ok type variable
    assert!(parser.parse("'a").unwrap() == ast::TypeExpression::TypeVariable(String::from("a")));
    assert!(parser.parse("'_yusdf").unwrap() == ast::TypeExpression::TypeVariable(String::from("_yusdf")));
    assert!(parser.parse("'aAbABBB").unwrap() == ast::TypeExpression::TypeVariable(String::from("aAbABBB")));
    assert!(parser.parse("'v1").unwrap() == ast::TypeExpression::TypeVariable(String::from("v1")));
    // Err type variable
    assert!(parser.parse("''").is_err());
    assert!(parser.parse("'hello'").is_err());
    assert!(parser.parse("'Type").is_err());
    assert!(parser.parse("'___Type").is_err());
    assert!(parser.parse("'950abc").is_err());
    assert!(parser.parse("8").is_err());
    // Ok list type
    assert!(parser.parse("[int]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::IntType
    )));
    assert!(parser.parse("[Option]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::DeclaredType(String::from("Option"), vec![])
    )));
    assert!(parser.parse("['a]").unwrap() == ast::TypeExpression::ListType(Box::new(
        ast::TypeExpression::TypeVariable(String::from("a"))
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
    // Err list type
    assert!(parser.parse("[int").is_err());
    assert!(parser.parse("[int, string]").is_err());
    // Ok tuple type
    assert!(parser.parse("('a, )").unwrap() == ast::TypeExpression::TupleType(vec![
        ast::TypeExpression::TypeVariable(String::from("a"))
    ]));
    assert!(parser.parse("(int, string, float)").unwrap() == ast::TypeExpression::TupleType(vec![
        ast::TypeExpression::IntType,
        ast::TypeExpression::StringType,
        ast::TypeExpression::FloatType
    ]));
    // Err tuple type
    assert!(parser.parse("()").is_err());
    assert!(parser.parse("int, string)").is_err());
    assert!(parser.parse("int, string, 'a").is_err());
    // Ok function type
    assert!(parser.parse("(int -> float)").unwrap() == ast::TypeExpression::FunctionType(vec![
        ast::TypeExpression::IntType,
        ast::TypeExpression::FloatType
    ]));
    assert!(parser.parse("('a -> 'b -> Option)").unwrap() == ast::TypeExpression::FunctionType(vec![
        ast::TypeExpression::TypeVariable(String::from("a")),
        ast::TypeExpression::TypeVariable(String::from("b")),
        ast::TypeExpression::DeclaredType(String::from("Option"), vec![])
    ]));
    // Err function type
    assert!(parser.parse("int ->").is_err());
    assert!(parser.parse("-> float").is_err());
    assert!(parser.parse("(int -> float").is_err());
    assert!(parser.parse("(int - > float)").is_err());
    assert!(parser.parse("(int -> 4)").is_err());
    // Ok declared type
    assert!(parser.parse("Bool").unwrap() == ast::TypeExpression::DeclaredType(String::from("Bool"), vec![]));
    assert!(parser.parse("Option int").unwrap() == ast::TypeExpression::DeclaredType(
        String::from("Option"),
        vec![ast::TypeExpression::IntType]
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
        vec![
            ast::TypeExpression::DeclaredType(String::from("Tree"), vec![
                ast::TypeExpression::FloatType
            ])
        ]
    ));
    // Err declared type
    assert!(parser.parse("Option \"hi\"").is_err());
    assert!(parser.parse("Tree Tree float").is_err());
    assert!(parser.parse("(Tree) float").is_err());
}

