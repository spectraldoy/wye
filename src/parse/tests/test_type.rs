use super::*;
use ast::Type::*;

#[test]
fn test_parse_literal() {
    let parser = grammar::TypeParser::new();
    assert!(parser.parse("none").unwrap() == None);
    assert!(parser.parse("int").unwrap() == Int);
    assert!(parser.parse("float").unwrap() == Float);
    assert!(parser.parse("string").unwrap() == String);
    assert!(parser.parse("iNT").unwrap() == TypeId("iNT".to_string(), vec![]));
    assert!(parser.parse("_x").unwrap() == TypeId("_x".to_string(), vec![]));
    assert!(parser.parse("flot").unwrap() == TypeId("flot".to_string(), vec![]));
    assert!(parser.parse("(int)").unwrap() == Int);
    assert!(parser.parse("(xello)").unwrap() == TypeId("xello".to_string(), vec![]));
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
    assert!(parser.parse("[int]").unwrap() == List(Box::new(Int)));
    assert!(
        parser.parse("[Option]").unwrap() == List(Box::new(TypeId("Option".to_string(), vec![])))
    );
    assert!(parser.parse("[(int)]").unwrap() == List(Box::new(Int)));
    assert!(
        parser.parse("[(int, Option)]").unwrap()
            == List(Box::new(Tuple(vec![
                Int,
                TypeId("Option".to_string(), vec![])
            ])))
    );
    assert!(parser.parse("[[int]]").unwrap() == List(Box::new(List(Box::new(Int)))));
    assert!(parser.parse("[int").is_err());
    assert!(parser.parse("[int, string]").is_err());
    assert!(parser.parse("hel]o").is_err());
    assert!(parser.parse("(int, string, float)").unwrap() == Tuple(vec![Int, String, Float]));
    assert!(
        parser.parse("(none, int, Option, string)").unwrap()
            == Tuple(vec![
                ast::Type::None,
                Int,
                TypeId("Option".to_string(), vec![]),
                String
            ])
    );
    assert!(parser.parse("()").is_err());
    assert!(parser.parse("int, string)").is_err());
    assert!(parser.parse("int, string, 'a").is_err());
    assert!(parser.parse("hi(, there").is_err());
    assert!(parser.parse("(xello, int, stri)ng, )").is_err());
    assert!(parser.parse("int string").is_err());
}

#[test]
fn test_parse_polymorphic_type() {
    let parser = grammar::TypeParser::new();

    assert!(
        parser.parse("'a").unwrap()
            == Poly {
                name: "a".to_string(),
                bound: Option::None
            }
    );
}

// TODO(WYE-6) uncomment these and test_parse_enum types and record types
// note that record types can have polytype variables to handle structs/interfaces
// that are parametrized with polytype variables

// #[test]
// fn test_parse_polymorphic_type() {
//     assert!(parser.parse("'a").unwrap() == UniversalType("a".to_string()));
//     assert!(
//         parser.parse("'_yusdf").unwrap()
//             == UniversalType("_yusdf".to_string())
//     );
//     assert!(
//         parser.parse("'aAbABBB").unwrap()
//             == UniversalType("aAbABBB".to_string())
//     );
//     assert!(parser.parse("'v1").unwrap() == UniversalType("v1".to_string()));
//     assert!(
//         parser.parse("'Type").unwrap() == UniversalType("Type".to_string())
//     );
//     assert!(
//         parser.parse("'___Type").unwrap()
//             == UniversalType("___Type".to_string())
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
//             == FunctionType(
//                 Box::new(Int),
//                 Box::new(Float)
//             )
//     );
//     assert!(
//         parser.parse("(int -> float)").unwrap()
//             == FunctionType(
//                 Box::new(Int),
//                 Box::new(Float)
//             )
//     );
//     assert!(
//         parser
//             .parse("(int->float->        string ->Option)")
//             .unwrap()
//             == FunctionType(
//                 Box::new(FunctionType(
//                     Box::new(FunctionType(
//                         Box::new(Int),
//                         Box::new(Float)
//                     )),
//                     Box::new(String)
//                 )),
//                 Box::new(DeclaredType(
//                     "Option".to_string(),
//                     vec![]
//                 ))
//             )
//     );
//     assert!(
//         parser.parse("'a -> 'b -> (int)").unwrap()
//             == FunctionType(
//                 Box::new(FunctionType(
//                     Box::new(UniversalType("a".to_string())),
//                     Box::new(UniversalType("b".to_string()))
//                 )),
//                 Box::new(Int)
//             )
//     );
//     assert!(
//         parser.parse("string -> Option int -> float").unwrap()
//             == FunctionType(
//                 Box::new(FunctionType(
//                     Box::new(String),
//                     Box::new(DeclaredType(
//                         "Option".to_string(),
//                         vec![Int]
//                     ))
//                 )),
//                 Box::new(Float)
//             )
//     );
//     assert!(
//         parser.parse("'a -> ('a -> int) -> 'a").unwrap()
//             == FunctionType(
//                 Box::new(FunctionType(
//                     Box::new(UniversalType("a".to_string())),
//                     Box::new(FunctionType(
//                         Box::new(UniversalType("a".to_string())),
//                         Box::new(Int)
//                     ))
//                 )),
//                 Box::new(UniversalType("a".to_string()))
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
//             == DeclaredType("bool".to_string(), vec![])
//     );
//     assert!(
//         parser.parse("Option int").unwrap()
//             == DeclaredType(
//                 "Option".to_string(),
//                 vec![Int]
//             )
//     );
//     assert!(
//         parser.parse("Tree (Tree) float").unwrap()
//             == DeclaredType(
//                 "Tree".to_string(),
//                 vec![
//                     DeclaredType("Tree".to_string(), vec![]),
//                     Float
//                 ]
//             )
//     );
//     assert!(
//         parser.parse("Tree (Tree float)").unwrap()
//             == DeclaredType(
//                 "Tree".to_string(),
//                 vec![DeclaredType(
//                     "Tree".to_string(),
//                     vec![Float]
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
