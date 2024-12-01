// use super::*;

// // Type Expressions

// #[test]
// fn test_parse_type_lit_univ_type() {
//     let parser = grammar::TypeExpressionParser::new();
//     // Ok literal type
//     assert!(parser.parse("int").unwrap() == ast::TypeExpression::IntType);
//     assert!(parser.parse("float").unwrap() == ast::TypeExpression::FloatType);
//     assert!(parser.parse("string").unwrap() == ast::TypeExpression::StringType);
//     assert!(
//         parser.parse("iNT").unwrap()
//             == ast::TypeExpression::DeclaredType(String::from("iNT"), vec![])
//     );
//     assert!(
//         parser.parse("_x").unwrap()
//             == ast::TypeExpression::DeclaredType(String::from("_x"), vec![])
//     );
//     assert!(
//         parser.parse("flot").unwrap()
//             == ast::TypeExpression::DeclaredType(String::from("flot"), vec![])
//     );
//     assert!(parser.parse("(int)").unwrap() == ast::TypeExpression::IntType);
//     assert!(
//         parser.parse("(xello)").unwrap()
//             == ast::TypeExpression::DeclaredType(String::from("xello"), vec![])
//     );
//     // Err literal type
//     assert!(parser.parse("Un[").is_err());
//     assert!(parser.parse("()").is_err());
//     // Ok type variable
//     assert!(parser.parse("'a").unwrap() == ast::TypeExpression::UniversalType(String::from("a")));
//     assert!(
//         parser.parse("'_yusdf").unwrap()
//             == ast::TypeExpression::UniversalType(String::from("_yusdf"))
//     );
//     assert!(
//         parser.parse("'aAbABBB").unwrap()
//             == ast::TypeExpression::UniversalType(String::from("aAbABBB"))
//     );
//     assert!(parser.parse("'v1").unwrap() == ast::TypeExpression::UniversalType(String::from("v1")));
//     assert!(
//         parser.parse("'Type").unwrap() == ast::TypeExpression::UniversalType(String::from("Type"))
//     );
//     assert!(
//         parser.parse("'___Type").unwrap()
//             == ast::TypeExpression::UniversalType(String::from("___Type"))
//     );
//     // Err type variable
//     assert!(parser.parse("''").is_err());
//     assert!(parser.parse("'hello'").is_err());
//     assert!(parser.parse("'950abc").is_err());
//     assert!(parser.parse("8").is_err());
//     assert!(parser.parse("\"hello\"").is_err());
//     assert!(parser.parse("x:int").is_err());
//     assert!(parser.parse("'aபாதை").is_err());
// }

// #[test]
// fn test_parse_list_tuple_type() {
//     let parser = grammar::TypeExpressionParser::new();
//     // Ok list type
//     assert!(
//         parser.parse("[int]").unwrap()
//             == ast::TypeExpression::ListType(Box::new(ast::TypeExpression::IntType))
//     );
//     assert!(
//         parser.parse("[Option]").unwrap()
//             == ast::TypeExpression::ListType(Box::new(ast::TypeExpression::DeclaredType(
//                 String::from("Option"),
//                 vec![]
//             )))
//     );
//     assert!(
//         parser.parse("['a]").unwrap()
//             == ast::TypeExpression::ListType(Box::new(ast::TypeExpression::UniversalType(
//                 String::from("a")
//             )))
//     );
//     assert!(
//         parser.parse("[(int)]").unwrap()
//             == ast::TypeExpression::ListType(Box::new(ast::TypeExpression::IntType))
//     );
//     assert!(
//         parser.parse("[(int, Option)]").unwrap()
//             == ast::TypeExpression::ListType(Box::new(ast::TypeExpression::TupleType(vec![
//                 ast::TypeExpression::IntType,
//                 ast::TypeExpression::DeclaredType(String::from("Option"), vec![])
//             ])))
//     );
//     assert!(
//         parser.parse("[[int]]").unwrap()
//             == ast::TypeExpression::ListType(Box::new(ast::TypeExpression::ListType(Box::new(
//                 ast::TypeExpression::IntType
//             ))))
//     );
//     assert!(
//         parser.parse("[Option int]").unwrap()
//             == ast::TypeExpression::ListType(Box::new(ast::TypeExpression::DeclaredType(
//                 String::from("Option"),
//                 vec![ast::TypeExpression::IntType]
//             )))
//     );
//     // Err list type
//     assert!(parser.parse("[int").is_err());
//     assert!(parser.parse("[int, string]").is_err());
//     assert!(parser.parse("hel]o").is_err());
//     // Ok tuple type
//     assert!(
//         parser.parse("('a, )").unwrap()
//             == ast::TypeExpression::TupleType(vec![ast::TypeExpression::UniversalType(
//                 String::from("a")
//             )])
//     );
//     assert!(
//         parser.parse("(int, string, float)").unwrap()
//             == ast::TypeExpression::TupleType(vec![
//                 ast::TypeExpression::IntType,
//                 ast::TypeExpression::StringType,
//                 ast::TypeExpression::FloatType
//             ])
//     );
//     assert!(
//         parser
//             .parse("(Option int float, int, Option, string)")
//             .unwrap()
//             == ast::TypeExpression::TupleType(vec![
//                 ast::TypeExpression::DeclaredType(
//                     String::from("Option"),
//                     vec![ast::TypeExpression::IntType, ast::TypeExpression::FloatType]
//                 ),
//                 ast::TypeExpression::IntType,
//                 ast::TypeExpression::DeclaredType(String::from("Option"), vec![]),
//                 ast::TypeExpression::StringType
//             ])
//     );
//     // Err tuple type
//     assert!(parser.parse("()").is_err());
//     assert!(parser.parse("int, string)").is_err());
//     assert!(parser.parse("int, string, 'a").is_err());
//     assert!(parser.parse("hi(, there").is_err());
//     assert!(parser.parse("(xello, int, stri)ng, )").is_err());
// }

// #[test]
// fn test_parse_function_type() {
//     let parser = grammar::TypeExpressionParser::new();
//     // Ok function type
//     assert!(
//         parser.parse("int -> float").unwrap()
//             == ast::TypeExpression::FunctionType(
//                 Box::new(ast::TypeExpression::IntType),
//                 Box::new(ast::TypeExpression::FloatType)
//             )
//     );
//     assert!(
//         parser.parse("(int -> float)").unwrap()
//             == ast::TypeExpression::FunctionType(
//                 Box::new(ast::TypeExpression::IntType),
//                 Box::new(ast::TypeExpression::FloatType)
//             )
//     );
//     assert!(
//         parser
//             .parse("(int->float->        string ->Option)")
//             .unwrap()
//             == ast::TypeExpression::FunctionType(
//                 Box::new(ast::TypeExpression::FunctionType(
//                     Box::new(ast::TypeExpression::FunctionType(
//                         Box::new(ast::TypeExpression::IntType),
//                         Box::new(ast::TypeExpression::FloatType)
//                     )),
//                     Box::new(ast::TypeExpression::StringType)
//                 )),
//                 Box::new(ast::TypeExpression::DeclaredType(
//                     String::from("Option"),
//                     vec![]
//                 ))
//             )
//     );
//     assert!(
//         parser.parse("'a -> 'b -> (int)").unwrap()
//             == ast::TypeExpression::FunctionType(
//                 Box::new(ast::TypeExpression::FunctionType(
//                     Box::new(ast::TypeExpression::UniversalType(String::from("a"))),
//                     Box::new(ast::TypeExpression::UniversalType(String::from("b")))
//                 )),
//                 Box::new(ast::TypeExpression::IntType)
//             )
//     );
//     assert!(
//         parser.parse("string -> Option int -> float").unwrap()
//             == ast::TypeExpression::FunctionType(
//                 Box::new(ast::TypeExpression::FunctionType(
//                     Box::new(ast::TypeExpression::StringType),
//                     Box::new(ast::TypeExpression::DeclaredType(
//                         String::from("Option"),
//                         vec![ast::TypeExpression::IntType]
//                     ))
//                 )),
//                 Box::new(ast::TypeExpression::FloatType)
//             )
//     );
//     assert!(
//         parser.parse("'a -> ('a -> int) -> 'a").unwrap()
//             == ast::TypeExpression::FunctionType(
//                 Box::new(ast::TypeExpression::FunctionType(
//                     Box::new(ast::TypeExpression::UniversalType(String::from("a"))),
//                     Box::new(ast::TypeExpression::FunctionType(
//                         Box::new(ast::TypeExpression::UniversalType(String::from("a"))),
//                         Box::new(ast::TypeExpression::IntType)
//                     ))
//                 )),
//                 Box::new(ast::TypeExpression::UniversalType(String::from("a")))
//             )
//     );
//     // Err function type
//     assert!(parser.parse("int ->").is_err());
//     assert!(parser.parse("-> float").is_err());
//     assert!(parser.parse("(int -> float").is_err());
//     assert!(parser.parse("(int - > float)").is_err());
//     assert!(parser.parse("(int -> 4)").is_err());
//     assert!(parser.parse("'a int -> int").is_err());
//     assert!(parser.parse("x: int -> y: float -> string").is_err());
// }

// #[test]
// fn test_parse_declared_type() {
//     let parser = grammar::TypeExpressionParser::new();
//     // Ok declared type
//     assert!(
//         parser.parse("bool").unwrap()
//             == ast::TypeExpression::DeclaredType(String::from("bool"), vec![])
//     );
//     assert!(
//         parser.parse("Option int").unwrap()
//             == ast::TypeExpression::DeclaredType(
//                 String::from("Option"),
//                 vec![ast::TypeExpression::IntType]
//             )
//     );
//     assert!(
//         parser.parse("Tree (Tree) float").unwrap()
//             == ast::TypeExpression::DeclaredType(
//                 String::from("Tree"),
//                 vec![
//                     ast::TypeExpression::DeclaredType(String::from("Tree"), vec![]),
//                     ast::TypeExpression::FloatType
//                 ]
//             )
//     );
//     assert!(
//         parser.parse("Tree (Tree float)").unwrap()
//             == ast::TypeExpression::DeclaredType(
//                 String::from("Tree"),
//                 vec![ast::TypeExpression::DeclaredType(
//                     String::from("Tree"),
//                     vec![ast::TypeExpression::FloatType]
//                 )]
//             )
//     );
//     // Err declared type
//     assert!(parser.parse("Option \"hi\"").is_err());
//     assert!(parser.parse("Tree Tree float").is_err());
//     assert!(parser.parse("(Tree) float").is_err());
//     assert!(parser.parse("(Tree) 'a").is_err());
//     assert!(parser.parse("bool'a").is_err());
//     assert!(parser.parse("(yello").is_err());
//     assert!(parser.parse("bool [int,]").is_err());
//     assert!(parser.parse("(yello").is_err());
//     assert!(parser.parse("X with int").is_err());
// }
