use super::*;

#[test]
fn test_parse_literal() {
    let parser = grammar::TypeParser::new();
    assert!(parser.parse(false, "none").unwrap() == ast::Type::None);
    assert!(parser.parse(false, "int").unwrap() == ast::Type::Int);
    assert!(parser.parse(false, "float").unwrap() == ast::Type::Float);
    assert!(parser.parse(false, "string").unwrap() == ast::Type::String);
    assert!(parser.parse(false, "iNT").unwrap() == ast::Type::TypeId("iNT".to_string(), vec![]));
    assert!(parser.parse(false, "_x").unwrap() == ast::Type::TypeId("_x".to_string(), vec![]));
    assert!(parser.parse(false, "flot").unwrap() == ast::Type::TypeId("flot".to_string(), vec![]));
    assert!(parser.parse(false, "(int)").unwrap() == ast::Type::Int);
    assert!(
        parser.parse(false, "(xello)").unwrap() == ast::Type::TypeId("xello".to_string(), vec![])
    );
    // Err literal type
    assert!(parser.parse(false, "Un[").is_err());
    assert!(parser.parse(false, "()").is_err());
    assert!(parser.parse(false, "8").is_err());
    assert!(parser.parse(false, "\"hello\"").is_err());
    assert!(parser.parse(false, "x:int").is_err());
    assert!(parser.parse(false, "nothing").is_err());
}

#[test]
fn test_parse_list_tuple_type() {
    let parser = grammar::TypeParser::new();
    assert!(parser.parse(false, "[int]").unwrap() == ast::Type::List(Box::new(ast::Type::Int)));
    assert!(
        parser.parse(false, "[Option]").unwrap()
            == ast::Type::List(Box::new(ast::Type::TypeId("Option".to_string(), vec![])))
    );
    assert!(parser.parse(false, "[(int)]").unwrap() == ast::Type::List(Box::new(ast::Type::Int)));
    assert!(
        parser.parse(false, "[(int, Option)]").unwrap()
            == ast::Type::List(Box::new(ast::Type::Tuple(vec![
                ast::Type::Int,
                ast::Type::TypeId("Option".to_string(), vec![])
            ])))
    );
    assert!(
        parser.parse(false, "[[int]]").unwrap()
            == ast::Type::List(Box::new(ast::Type::List(Box::new(ast::Type::Int))))
    );
    assert!(parser.parse(false, "[int").is_err());
    assert!(parser.parse(false, "[int, string]").is_err());
    assert!(parser.parse(false, "hel]o").is_err());
    assert!(
        parser.parse(false, "(int, string, float)").unwrap()
            == ast::Type::Tuple(vec![ast::Type::Int, ast::Type::String, ast::Type::Float])
    );
    assert!(
        parser.parse(false, "(none, int, Option, string)").unwrap()
            == ast::Type::Tuple(vec![
                ast::Type::None,
                ast::Type::Int,
                ast::Type::TypeId("Option".to_string(), vec![]),
                ast::Type::String
            ])
    );
    assert!(parser.parse(false, "()").is_err());
    assert!(parser.parse(false, "int, string)").is_err());
    assert!(parser.parse(false, "int, string, 'a").is_err());
    assert!(parser.parse(false, "hi(, there").is_err());
    assert!(parser.parse(false, "(xello, int, stri)ng, )").is_err());
    assert!(parser.parse(false, "int string").is_err());
}

// TODO(WYE-6) uncomment these and test_parse_enum types and record types
// note that record types can have polytype variables to handle structs/interfaces
// that are parametrized with polytype variables

// #[test]
// fn test_parse_polymorphic_type() {
//     assert!(parser.parse(false, "'a").unwrap() == ast::Type::UniversalType("a".to_string()));
//     assert!(
//         parser.parse(false, "'_yusdf").unwrap()
//             == ast::Type::UniversalType("_yusdf".to_string())
//     );
//     assert!(
//         parser.parse(false, "'aAbABBB").unwrap()
//             == ast::Type::UniversalType("aAbABBB".to_string())
//     );
//     assert!(parser.parse(false, "'v1").unwrap() == ast::Type::UniversalType("v1".to_string()));
//     assert!(
//         parser.parse(false, "'Type").unwrap() == ast::Type::UniversalType("Type".to_string())
//     );
//     assert!(
//         parser.parse(false, "'___Type").unwrap()
//             == ast::Type::UniversalType("___Type".to_string())
//     );
//     assert!(parser.parse(false, "''").is_err());
//     assert!(parser.parse(false, "'hello'").is_err());
//     assert!(parser.parse(false, "'950abc").is_err());
//     assert!(parser.parse(false, "'aபாதை").is_err());
// }

// #[test]
// fn test_parse_function_type() {
//     let parser = grammar::TypeParser::new();
//     // Ok function type
//     assert!(
//         parser.parse(false, "int -> float").unwrap()
//             == ast::Type::FunctionType(
//                 Box::new(ast::Type::Int),
//                 Box::new(ast::Type::Float)
//             )
//     );
//     assert!(
//         parser.parse(false, "(int -> float)").unwrap()
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
//         parser.parse(false, "'a -> 'b -> (int)").unwrap()
//             == ast::Type::FunctionType(
//                 Box::new(ast::Type::FunctionType(
//                     Box::new(ast::Type::UniversalType("a".to_string())),
//                     Box::new(ast::Type::UniversalType("b".to_string()))
//                 )),
//                 Box::new(ast::Type::Int)
//             )
//     );
//     assert!(
//         parser.parse(false, "string -> Option int -> float").unwrap()
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
//         parser.parse(false, "'a -> ('a -> int) -> 'a").unwrap()
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
//     assert!(parser.parse(false, "int ->").is_err());
//     assert!(parser.parse(false, "-> float").is_err());
//     assert!(parser.parse(false, "(int -> float").is_err());
//     assert!(parser.parse(false, "(int - > float)").is_err());
//     assert!(parser.parse(false, "(int -> 4)").is_err());
//     assert!(parser.parse(false, "'a int -> int").is_err());
//     assert!(parser.parse(false, "x: int -> y: float -> string").is_err());
// }

// #[test]
// fn test_parse_declared_type() {
//     let parser = grammar::TypeParser::new();
//     // Ok declared type
//     assert!(
//         parser.parse(false, "bool").unwrap()
//             == ast::Type::DeclaredType("bool".to_string(), vec![])
//     );
//     assert!(
//         parser.parse(false, "Option int").unwrap()
//             == ast::Type::DeclaredType(
//                 "Option".to_string(),
//                 vec![ast::Type::Int]
//             )
//     );
//     assert!(
//         parser.parse(false, "Tree (Tree) float").unwrap()
//             == ast::Type::DeclaredType(
//                 "Tree".to_string(),
//                 vec![
//                     ast::Type::DeclaredType("Tree".to_string(), vec![]),
//                     ast::Type::Float
//                 ]
//             )
//     );
//     assert!(
//         parser.parse(false, "Tree (Tree float)").unwrap()
//             == ast::Type::DeclaredType(
//                 "Tree".to_string(),
//                 vec![ast::Type::DeclaredType(
//                     "Tree".to_string(),
//                     vec![ast::Type::Float]
//                 )]
//             )
//     );
//     // Err declared type
//     assert!(parser.parse(false, "Option \"hi\"").is_err());
//     assert!(parser.parse(false, "Tree Tree float").is_err());
//     assert!(parser.parse(false, "(Tree) float").is_err());
//     assert!(parser.parse(false, "(Tree) 'a").is_err());
//     assert!(parser.parse(false, "bool'a").is_err());
//     assert!(parser.parse(false, "(yello").is_err());
//     assert!(parser.parse(false, "bool [int,]").is_err());
//     assert!(parser.parse(false, "(yello").is_err());
//     assert!(parser.parse(false, "X with int").is_err());
// }
