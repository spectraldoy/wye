// use super::*;

// // Patterns

// #[test]
// fn test_parse_atomic_pattern() {
//     let parser = grammar::PatternParser::new();
//     // Ok atomic
//     assert!(parser.parse("_").unwrap() == ast::Pattern::Wildcard);
//     assert!(parser.parse("__").unwrap() == ast::Pattern::Identifier(String::from("__")));
//     assert!(parser.parse("-6789").unwrap() == ast::Pattern::IntegerLiteral(-6789));
//     assert!(parser.parse("9.8e3").unwrap() == ast::Pattern::FloatLiteral(OrderedFloat(9800.0)));
//     assert!(
//         parser.parse("\"helloⓓⓕ\"").unwrap()
//             == ast::Pattern::StringLiteral(String::from("helloⓓⓕ"))
//     );
//     assert!(parser.parse("x").unwrap() == ast::Pattern::Identifier(String::from("x")));
//     assert!(parser.parse("__o98").unwrap() == ast::Pattern::Identifier(String::from("__o98")));
//     assert!(
//         parser.parse("Some with x").unwrap()
//             == ast::Pattern::TypeVariant(
//                 String::from("Some"),
//                 Box::new(ast::Pattern::Identifier(String::from("x")))
//             )
//     );
//     assert!(
//         parser.parse("Option with (Tree, 4)").unwrap()
//             == ast::Pattern::TypeVariant(
//                 String::from("Option"),
//                 Box::new(ast::Pattern::Tuple(vec![
//                     ast::Pattern::Identifier(String::from("Tree")),
//                     ast::Pattern::IntegerLiteral(4)
//                 ]))
//             )
//     );
//     assert!(
//         parser.parse("Some with 4").unwrap()
//             == ast::Pattern::TypeVariant(
//                 String::from("Some"),
//                 Box::new(ast::Pattern::IntegerLiteral(4))
//             )
//     );
//     assert!(parser.parse("[ ]").unwrap() == ast::Pattern::EmptyList);
//     assert!(
//         parser.parse("bool with (List with [_, x :: xs])").unwrap()
//             == ast::Pattern::TypeVariant(
//                 String::from("bool"),
//                 Box::new(ast::Pattern::TypeVariant(
//                     String::from("List"),
//                     Box::new(ast::Pattern::List(vec![
//                         ast::Pattern::Wildcard,
//                         ast::Pattern::ListConstruction(String::from("x"), String::from("xs"))
//                     ]))
//                 ))
//             )
//     );
//     assert!(
//         parser.parse("Integer with (4)").unwrap()
//             == ast::Pattern::TypeVariant(
//                 String::from("Integer"),
//                 Box::new(ast::Pattern::IntegerLiteral(4))
//             )
//     );
//     // Err atomic
//     assert!(parser.parse("=>").is_err());
//     assert!(parser.parse("ⓓⓕ").is_err());
//     assert!(parser.parse("- 5").is_err());
//     assert!(parser.parse("__ => _").is_err());
//     assert!(parser.parse("98x").is_err());
//     assert!(parser.parse("Some with int").is_err());
//     assert!(parser.parse("Option x").is_err());
//     assert!(parser.parse("Thingy with ([x, y])").is_err());
// }

// #[test]
// fn test_parse_compound_pattern() {
//     let parser = grammar::PatternParser::new();
//     // Ok compound
//     assert!(
//         parser.parse("[4, _, _, []]").unwrap()
//             == ast::Pattern::List(vec![
//                 ast::Pattern::IntegerLiteral(4),
//                 ast::Pattern::Wildcard,
//                 ast::Pattern::Wildcard,
//                 ast::Pattern::EmptyList
//             ])
//     );
//     assert!(
//         parser
//             .parse("Integer with (Option with y, x :: xs)")
//             .unwrap()
//             == ast::Pattern::TypeVariant(
//                 String::from("Integer"),
//                 Box::new(ast::Pattern::Tuple(vec![
//                     ast::Pattern::TypeVariant(
//                         String::from("Option"),
//                         Box::new(ast::Pattern::Identifier(String::from("y")))
//                     ),
//                     ast::Pattern::ListConstruction(String::from("x"), String::from("xs"))
//                 ]))
//             )
//     );
//     assert!(
//         parser.parse("Float with (5.7,)").unwrap()
//             == ast::Pattern::TypeVariant(
//                 String::from("Float"),
//                 Box::new(ast::Pattern::Tuple(vec![ast::Pattern::FloatLiteral(
//                     OrderedFloat(5.7)
//                 )]))
//             )
//     );
//     assert!(
//         parser.parse("(_, _, v)").unwrap()
//             == ast::Pattern::Tuple(vec![
//                 ast::Pattern::Wildcard,
//                 ast::Pattern::Wildcard,
//                 ast::Pattern::Identifier(String::from("v"))
//             ])
//     );
//     assert!(
//         parser.parse("[4, -5.6, _]").unwrap()
//             == ast::Pattern::List(vec![
//                 ast::Pattern::IntegerLiteral(4),
//                 ast::Pattern::FloatLiteral(OrderedFloat(-5.6)),
//                 ast::Pattern::Wildcard
//             ])
//     );
//     assert!(
//         parser.parse("(7, )").unwrap()
//             == ast::Pattern::Tuple(vec![ast::Pattern::IntegerLiteral(7)])
//     );
//     // Err compound
//     assert!(parser.parse("(_, 4").is_err());
//     assert!(parser.parse("( )").is_err());
//     assert!(parser.parse("[x, y").is_err());
//     assert!(parser.parse("[4,]").is_err());
//     assert!(parser.parse("(x)with y").is_err());
//     assert!(parser.parse("with 4").is_err());
//     assert!(parser.parse("[[Some with x]]").is_err());
//     assert!(parser.parse("(x, _, (4, 5))").is_err());
//     assert!(parser.parse("([4, -5.6, _], (7, ), x, )").is_err());
// }

// #[test]
// fn test_parse_complex_pattern() {
//     let parser = grammar::PatternParser::new();
//     // Ok Pattern union
//     assert!(
//         parser.parse("x | y").unwrap()
//             == ast::Pattern::Union(vec![
//                 ast::Pattern::Identifier(String::from("x")),
//                 ast::Pattern::Identifier(String::from("y"))
//             ])
//     );
//     assert!(
//         parser.parse("(4) | 5.0 | \"hello\"").unwrap()
//             == ast::Pattern::Union(vec![
//                 ast::Pattern::IntegerLiteral(4),
//                 ast::Pattern::FloatLiteral(OrderedFloat(5.0)),
//                 ast::Pattern::StringLiteral(String::from("hello"))
//             ])
//     );
//     assert!(
//         parser.parse("x|(y)").unwrap()
//             == ast::Pattern::Union(vec![
//                 ast::Pattern::Identifier(String::from("x")),
//                 ast::Pattern::Identifier(String::from("y"))
//             ])
//     );
//     // Err pattern union
//     assert!(parser.parse("[a, b] | 4").is_err());
//     assert!(parser.parse("b|(x,)").is_err());
//     assert!(parser.parse("4 |").is_err());
//     assert!(parser.parse("|").is_err());
//     assert!(parser.parse("4 | 5 | ").is_err());
//     assert!(parser.parse("| 6.0 | 7").is_err());
//     assert!(parser.parse("([x, y]) | (4)").is_err());
//     // Ok Pattern complement
//     assert!(
//         parser.parse("~4").unwrap()
//             == ast::Pattern::Complement(Box::new(ast::Pattern::IntegerLiteral(4)))
//     );
//     assert!(
//         parser.parse("~[_, 4]").unwrap()
//             == ast::Pattern::Complement(Box::new(ast::Pattern::List(vec![
//                 ast::Pattern::Wildcard,
//                 ast::Pattern::IntegerLiteral(4)
//             ])))
//     );
//     assert!(
//         parser.parse("~Option with 4").unwrap()
//             == ast::Pattern::Complement(Box::new(ast::Pattern::TypeVariant(
//                 String::from("Option"),
//                 Box::new(ast::Pattern::IntegerLiteral(4))
//             )))
//     );
//     assert!(
//         parser.parse("~(Option with (2, 3, x, ))").unwrap()
//             == ast::Pattern::Complement(Box::new(ast::Pattern::TypeVariant(
//                 String::from("Option"),
//                 Box::new(ast::Pattern::Tuple(vec![
//                     ast::Pattern::IntegerLiteral(2),
//                     ast::Pattern::IntegerLiteral(3),
//                     ast::Pattern::Identifier(String::from("x"))
//                 ]))
//             )))
//     );
//     // Err Pattern complement
//     assert!(parser.parse("!").is_err());
//     assert!(parser.parse("~").is_err());
//     assert!(parser.parse("~~4").is_err());
//     assert!(parser.parse("~(~4)").is_err());
//     assert!(parser.parse("4~").is_err());
//     assert!(parser.parse("5 | ~6").is_err());
//     assert!(parser.parse("~ 5 | 4").is_err());
//     // Ok guarded pattern
//     assert!(
//         parser.parse("x | y if true").unwrap()
//             == ast::Pattern::Guarded(
//                 Box::new(ast::Pattern::Union(vec![
//                     ast::Pattern::Identifier(String::from("x")),
//                     ast::Pattern::Identifier(String::from("y"))
//                 ])),
//                 ast::Expression::Identifier(String::from("true"))
//             )
//     );
//     assert!(
//         parser.parse("x :: xs if f x").unwrap()
//             == ast::Pattern::Guarded(
//                 Box::new(ast::Pattern::ListConstruction(
//                     String::from("x"),
//                     String::from("xs")
//                 )),
//                 ast::Expression::FuncApplication(
//                     Box::new(ast::Expression::Identifier(String::from("f"))),
//                     Box::new(ast::Expression::Identifier(String::from("x")))
//                 )
//             )
//     );
//     // Err guarded pattern
//     assert!(parser.parse("if").is_err());
//     assert!(parser.parse("x | 4if true").is_err());
//     assert!(parser.parse("x | yif true").is_err());
//     assert!(parser.parse("4 | 5 if(f g)").is_err());
//     assert!(parser.parse("[a, b] if").is_err());
//     assert!(parser.parse("(f g)").is_err());
// }
