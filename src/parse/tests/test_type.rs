use super::*;

#[test]
fn test_parse_literal() {
    let parser = grammar::TypeParser::new();
    assert!(parser.parse("none").unwrap() == ast::Type::None);
    assert!(parser.parse("int").unwrap() == ast::Type::Int);
    assert!(parser.parse("float").unwrap() == ast::Type::Float);
    assert!(parser.parse("string").unwrap() == ast::Type::String);
    assert!(parser.parse("iNT").unwrap() == ast::Type::TypeId("iNT".to_string(), vec![]));
    assert!(parser.parse("_x").unwrap() == ast::Type::TypeId("_x".to_string(), vec![]));
    assert!(parser.parse("flot").unwrap() == ast::Type::TypeId("flot".to_string(), vec![]));
    assert!(parser.parse("(int)").unwrap() == ast::Type::Int);
    assert!(parser.parse("(xello)").unwrap() == ast::Type::TypeId("xello".to_string(), vec![]));
    // Err literal type
    assert!(parser.parse("Un[").is_err());
    assert!(parser.parse("()").is_err());
    assert!(parser.parse("8").is_err());
    assert!(parser.parse("\"hello\"").is_err());
    assert!(parser.parse("x:int").is_err());
    assert!(parser.parse("nothing").is_err());
}

#[test]
fn test_parse_list_tuple_type() {
    let parser = grammar::TypeParser::new();
    assert!(parser.parse("[int]").unwrap() == ast::Type::List(Box::new(ast::Type::Int)));
    assert!(
        parser.parse("[Option]").unwrap()
            == ast::Type::List(Box::new(ast::Type::TypeId("Option".to_string(), vec![])))
    );
    assert!(parser.parse("[(int)]").unwrap() == ast::Type::List(Box::new(ast::Type::Int)));
    assert!(
        parser.parse("[(int, Option)]").unwrap()
            == ast::Type::List(Box::new(ast::Type::Tuple(vec![
                ast::Type::Int,
                ast::Type::TypeId("Option".to_string(), vec![])
            ])))
    );
    assert!(
        parser.parse("[[int]]").unwrap()
            == ast::Type::List(Box::new(ast::Type::List(Box::new(ast::Type::Int))))
    );
    assert!(parser.parse("[int").is_err());
    assert!(parser.parse("[int, string]").is_err());
    assert!(parser.parse("hel]o").is_err());
    assert!(
        parser.parse("(int, string, float)").unwrap()
            == ast::Type::Tuple(vec![ast::Type::Int, ast::Type::String, ast::Type::Float])
    );
    assert!(
        parser.parse("(none, int, Option, string)").unwrap()
            == ast::Type::Tuple(vec![
                ast::Type::None,
                ast::Type::Int,
                ast::Type::TypeId("Option".to_string(), vec![]),
                ast::Type::String
            ])
    );
    assert!(parser.parse("()").is_err());
    assert!(parser.parse("int, string)").is_err());
    assert!(parser.parse("int, string, 'a").is_err());
    assert!(parser.parse("hi(, there").is_err());
    assert!(parser.parse("(xello, int, stri)ng, )").is_err());
    assert!(parser.parse("int string").is_err());
}

// TODO(WYE-6) uncomment these and test_parse_enum types and record types
// note that record types can have polytype variables to handle structs/interfaces
// that are parametrized with polytype variables

// #[test]
// fn test_parse_polymorphic_type() {
//     assert!(parser.parse("'a").unwrap() == ast::Type::UniversalType("a".to_string()));
//     assert!(
//         parser.parse("'_yusdf").unwrap()
//             == ast::Type::UniversalType("_yusdf".to_string())
//     );
//     assert!(
//         parser.parse("'aAbABBB").unwrap()
//             == ast::Type::UniversalType("aAbABBB".to_string())
//     );
//     assert!(parser.parse("'v1").unwrap() == ast::Type::UniversalType("v1".to_string()));
//     assert!(
//         parser.parse("'Type").unwrap() == ast::Type::UniversalType("Type".to_string())
//     );
//     assert!(
//         parser.parse("'___Type").unwrap()
//             == ast::Type::UniversalType("___Type".to_string())
//     );
//     assert!(parser.parse("''").is_err());
//     assert!(parser.parse("'hello'").is_err());
//     assert!(parser.parse("'950abc").is_err());
//     assert!(parser.parse("'aபாதை").is_err());
// }

// #[test]
// fn test_parse_function_type() {
//     let parser = grammar::TypeParser::new();
//     // Ok function type
//     assert!(
//         parser.parse("int -> float").unwrap()
//             == ast::Type::FunctionType(
//                 Box::new(ast::Type::Int),
//                 Box::new(ast::Type::Float)
//             )
//     );
//     assert!(
//         parser.parse("(int -> float)").unwrap()
//             == ast::Type::FunctionType(
//                 Box::new(ast::Type::Int),
//                 Box::new(ast::Type::Float)
//             )
//     );
//     assert!(
//         parser
//             .parse("(int->float->        string ->Option)")
//             .unwrap()
//             == ast::Type::FunctionType(
//                 Box::new(ast::Type::FunctionType(
//                     Box::new(ast::Type::FunctionType(
//                         Box::new(ast::Type::Int),
//                         Box::new(ast::Type::Float)
//                     )),
//                     Box::new(ast::Type::String)
//                 )),
//                 Box::new(ast::Type::DeclaredType(
//                     "Option".to_string(),
//                     vec![]
//                 ))
//             )
//     );
//     assert!(
//         parser.parse("'a -> 'b -> (int)").unwrap()
//             == ast::Type::FunctionType(
//                 Box::new(ast::Type::FunctionType(
//                     Box::new(ast::Type::UniversalType("a".to_string())),
//                     Box::new(ast::Type::UniversalType("b".to_string()))
//                 )),
//                 Box::new(ast::Type::Int)
//             )
//     );
//     assert!(
//         parser.parse("string -> Option int -> float").unwrap()
//             == ast::Type::FunctionType(
//                 Box::new(ast::Type::FunctionType(
//                     Box::new(ast::Type::String),
//                     Box::new(ast::Type::DeclaredType(
//                         "Option".to_string(),
//                         vec![ast::Type::Int]
//                     ))
//                 )),
//                 Box::new(ast::Type::Float)
//             )
//     );
//     assert!(
//         parser.parse("'a -> ('a -> int) -> 'a").unwrap()
//             == ast::Type::FunctionType(
//                 Box::new(ast::Type::FunctionType(
//                     Box::new(ast::Type::UniversalType("a".to_string())),
//                     Box::new(ast::Type::FunctionType(
//                         Box::new(ast::Type::UniversalType("a".to_string())),
//                         Box::new(ast::Type::Int)
//                     ))
//                 )),
//                 Box::new(ast::Type::UniversalType("a".to_string()))
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
//     let parser = grammar::TypeParser::new();
//     // Ok declared type
//     assert!(
//         parser.parse("bool").unwrap()
//             == ast::Type::DeclaredType("bool".to_string(), vec![])
//     );
//     assert!(
//         parser.parse("Option int").unwrap()
//             == ast::Type::DeclaredType(
//                 "Option".to_string(),
//                 vec![ast::Type::Int]
//             )
//     );
//     assert!(
//         parser.parse("Tree (Tree) float").unwrap()
//             == ast::Type::DeclaredType(
//                 "Tree".to_string(),
//                 vec![
//                     ast::Type::DeclaredType("Tree".to_string(), vec![]),
//                     ast::Type::Float
//                 ]
//             )
//     );
//     assert!(
//         parser.parse("Tree (Tree float)").unwrap()
//             == ast::Type::DeclaredType(
//                 "Tree".to_string(),
//                 vec![ast::Type::DeclaredType(
//                     "Tree".to_string(),
//                     vec![ast::Type::Float]
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
