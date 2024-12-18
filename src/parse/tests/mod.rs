use super::*;

#[cfg(test)]
mod test_expr;
#[cfg(test)]
mod test_pattern;
#[cfg(test)]
mod test_type;
#[cfg(test)]
mod util;

// Statements

// #[test]
// fn test_parse_type_decl() {
//     let parser = grammar::StatementParser::new();
//     // Ok
//     assert!(
//         parser.parse("type bool = false | true;").unwrap()
//             == ast::Statement::TypeDeclaration(
//                 String::from("bool"),
//                 vec![],
//                 vec![(String::from("false"), None), (String::from("true"), None)]
//             )
//     );
//     assert!(
//         parser
//             .parse("type Option 'a = None | Some with 'a;")
//             .unwrap()
//             == ast::Statement::TypeDeclaration(
//                 String::from("Option"),
//                 vec![String::from("a")],
//                 vec![
//                     (String::from("None"), None),
//                     (
//                         String::from("Some"),
//                         Some(ast::TypeExpression::UniversalType(String::from("a")))
//                     )
//                 ]
//             )
//     );
//     assert!(
//         parser
//             .parse(
//                 "type Thingy = Var1
//         | Var2 with int
//         | Var3 with string
//         | Var4 with [int]
//         |Var5 with (int, string, Thingy)
//     ;"
//             )
//             .unwrap()
//             == ast::Statement::TypeDeclaration(
//                 String::from("Thingy"),
//                 vec![],
//                 vec![
//                     (String::from("Var1"), None),
//                     (String::from("Var2"), Some(ast::TypeExpression::IntType)),
//                     (String::from("Var3"), Some(ast::TypeExpression::StringType)),
//                     (
//                         String::from("Var4"),
//                         Some(ast::TypeExpression::ListType(Box::new(
//                             ast::TypeExpression::IntType
//                         )))
//                     ),
//                     (
//                         String::from("Var5"),
//                         Some(ast::TypeExpression::TupleType(vec![
//                             ast::TypeExpression::IntType,
//                             ast::TypeExpression::StringType,
//                             ast::TypeExpression::DeclaredType(String::from("Thingy"), vec![])
//                         ]))
//                     )
//                 ]
//             )
//     );
//     assert!(
//         parser
//             .parse(
//                 "
//     type binary_tree 'a = Leaf
//         | Node with ('a, binary_tree 'a, binary_tree 'a);"
//             )
//             .unwrap()
//             == ast::Statement::TypeDeclaration(
//                 String::from("binary_tree"),
//                 vec![String::from("a")],
//                 vec![
//                     (String::from("Leaf"), None),
//                     (
//                         String::from("Node"),
//                         Some(ast::TypeExpression::TupleType(vec![
//                             ast::TypeExpression::UniversalType(String::from("a")),
//                             ast::TypeExpression::DeclaredType(
//                                 String::from("binary_tree"),
//                                 vec![ast::TypeExpression::UniversalType(String::from("a"))]
//                             ),
//                             ast::TypeExpression::DeclaredType(
//                                 String::from("binary_tree"),
//                                 vec![ast::TypeExpression::UniversalType(String::from("a"))]
//                             )
//                         ]))
//                     )
//                 ]
//             )
//     );
//     assert!(
//         parser.parse("type Onevar = Onevar;").unwrap()
//             == ast::Statement::TypeDeclaration(
//                 String::from("Onevar"),
//                 vec![],
//                 vec![(String::from("Onevar"), None)]
//             )
//     );
//     // Err
//     assert!(parser
//         .parse("type tvar = X with [int, string] | Y;")
//         .is_err());
//     assert!(parser
//         .parse("type Option 'a -> 'b = None | Some with ('a, 'b)")
//         .is_err());
//     assert!(parser.parse("type Tree 'a = Leaf | Node + 'a;").is_err());
//     assert!(parser.parse("typebool = false | true;").is_err());
//     assert!(parser
//         .parse("type bool 'a = match 'a { true => true, false => false };")
//         .is_err());
// }

// // Program

// #[test]
// fn test_parse_program() {
//     let parser = grammar::ProgramParser::new();
//     // Ok
//     assert!(
//         parser
//             .parse(
//                 "
//         let f (g: 'a -> 'a) -> (x: 'a) -> 'a = g x;
//         let double x = (*) 2 x;
//         let z = f double 4;
//     "
//             )
//             .unwrap()
//             == vec![
//                 ast::Statement::TypedLet(
//                     String::from("f"),
//                     ast::TypeExpression::UniversalType(String::from("a")),
//                     vec![
//                         (
//                             String::from("g"),
//                             ast::TypeExpression::FunctionType(
//                                 Box::new(ast::TypeExpression::UniversalType(String::from("a"))),
//                                 Box::new(ast::TypeExpression::UniversalType(String::from("a")))
//                             )
//                         ),
//                         (
//                             String::from("x"),
//                             ast::TypeExpression::UniversalType(String::from("a"))
//                         )
//                     ],
//                     ast::Expression::FuncApplication(
//                         Box::new(ast::Expression::Identifier(String::from("g"))),
//                         Box::new(ast::Expression::Identifier(String::from("x")))
//                     )
//                 ),
//                 ast::Statement::UntypedLet(
//                     vec![String::from("double"), String::from("x")],
//                     ast::Expression::FuncApplication(
//                         Box::new(ast::Expression::FuncApplication(
//                             Box::new(ast::Expression::BuiltinOp(ast::Operation::Multiply)),
//                             Box::new(ast::Expression::IntegerLiteral(2))
//                         )),
//                         Box::new(ast::Expression::Identifier(String::from("x")))
//                     )
//                 ),
//                 ast::Statement::UntypedLet(
//                     vec![String::from("z")],
//                     ast::Expression::FuncApplication(
//                         Box::new(ast::Expression::FuncApplication(
//                             Box::new(ast::Expression::Identifier(String::from("f"))),
//                             Box::new(ast::Expression::Identifier(String::from("double")))
//                         )),
//                         Box::new(ast::Expression::IntegerLiteral(4))
//                     )
//                 )
//             ]
//     );
//     assert!(
//         parser
//             .parse(
//                 "
//         type Option 'a = None | Some with 'a;
//         let print_optional (x: Option 'a) -> string = match x {
//             None => print \"nothing!\",
//             Some with x => print (\"something: \" + x)
//         };
//     "
//             )
//             .unwrap()
//             == vec![
//                 ast::Statement::TypeDeclaration(
//                     String::from("Option"),
//                     vec![String::from("a")],
//                     vec![
//                         (String::from("None"), None),
//                         (
//                             String::from("Some"),
//                             Some(ast::TypeExpression::UniversalType(String::from("a")))
//                         )
//                     ]
//                 ),
//                 ast::Statement::TypedLet(
//                     String::from("print_optional"),
//                     ast::TypeExpression::StringType,
//                     vec![(
//                         String::from("x"),
//                         ast::TypeExpression::DeclaredType(
//                             String::from("Option"),
//                             vec![ast::TypeExpression::UniversalType(String::from("a"))]
//                         )
//                     )],
//                     ast::Expression::MatchConstruct(
//                         Box::new(ast::Expression::Identifier(String::from("x"))),
//                         vec![
//                             (
//                                 ast::Pattern::Identifier(String::from("None")),
//                                 ast::Expression::FuncApplication(
//                                     Box::new(ast::Expression::Print),
//                                     Box::new(ast::Expression::StringLiteral(String::from(
//                                         "nothing!"
//                                     )))
//                                 )
//                             ),
//                             (
//                                 ast::Pattern::TypeVariant(
//                                     String::from("Some"),
//                                     Box::new(ast::Pattern::Identifier(String::from("x")))
//                                 ),
//                                 ast::Expression::FuncApplication(
//                                     Box::new(ast::Expression::Print),
//                                     Box::new(ast::Expression::FuncApplication(
//                                         Box::new(ast::Expression::FuncApplication(
//                                             Box::new(ast::Expression::BuiltinOp(
//                                                 ast::Operation::Add
//                                             )),
//                                             Box::new(ast::Expression::StringLiteral(String::from(
//                                                 "something: "
//                                             )))
//                                         )),
//                                         Box::new(ast::Expression::Identifier(String::from("x")))
//                                     ))
//                                 )
//                             )
//                         ]
//                     )
//                 )
//             ]
//     );
//     assert!(
//         parser
//             .parse(
//                 "
//         type OptionalTuple 'a 'b = None | Some with ('a, 'b);
//         let f (x: 'a) -> (y: 'b) -> OptionalTuple 'a 'b = Some with (x, y);
//     "
//             )
//             .unwrap()
//             == vec![
//                 ast::Statement::TypeDeclaration(
//                     String::from("OptionalTuple"),
//                     vec![String::from("a"), String::from("b")],
//                     vec![
//                         (String::from("None"), None),
//                         (
//                             String::from("Some"),
//                             Some(ast::TypeExpression::TupleType(vec![
//                                 ast::TypeExpression::UniversalType(String::from("a")),
//                                 ast::TypeExpression::UniversalType(String::from("b"))
//                             ]))
//                         )
//                     ]
//                 ),
//                 ast::Statement::TypedLet(
//                     String::from("f"),
//                     ast::TypeExpression::DeclaredType(
//                         String::from("OptionalTuple"),
//                         vec![
//                             ast::TypeExpression::UniversalType(String::from("a")),
//                             ast::TypeExpression::UniversalType(String::from("b"))
//                         ]
//                     ),
//                     vec![
//                         (
//                             String::from("x"),
//                             ast::TypeExpression::UniversalType(String::from("a"))
//                         ),
//                         (
//                             String::from("y"),
//                             ast::TypeExpression::UniversalType(String::from("b"))
//                         )
//                     ],
//                     ast::Expression::TypeVariant(
//                         String::from("Some"),
//                         Box::new(ast::Expression::Tuple(vec![
//                             ast::Expression::Identifier(String::from("x")),
//                             ast::Expression::Identifier(String::from("y"))
//                         ]))
//                     )
//                 )
//             ]
//     );
//     assert!(
//         parser
//             .parse(
//                 "
//         type Option 'a = Some with 'a | None;
//         let x: Option int = Some with 4;
//     "
//             )
//             .unwrap()
//             == vec![
//                 ast::Statement::TypeDeclaration(
//                     String::from("Option"),
//                     vec![String::from("a")],
//                     vec![
//                         (
//                             String::from("Some"),
//                             Some(ast::TypeExpression::UniversalType(String::from("a")))
//                         ),
//                         (String::from("None"), None)
//                     ]
//                 ),
//                 ast::Statement::TypedLet(
//                     String::from("x"),
//                     ast::TypeExpression::DeclaredType(
//                         String::from("Option"),
//                         vec![ast::TypeExpression::IntType]
//                     ),
//                     vec![],
//                     ast::Expression::TypeVariant(
//                         String::from("Some"),
//                         Box::new(ast::Expression::IntegerLiteral(4))
//                     )
//                 )
//             ]
//     );
//     assert!(
//         parser
//             .parse(
//                 "
//         type Option 'a = Some with 'a | None;
//         let x: Option int = {
//             let y = 4;
//             Some with y
//         };
//     "
//             )
//             .unwrap()
//             == vec![
//                 ast::Statement::TypeDeclaration(
//                     String::from("Option"),
//                     vec![String::from("a")],
//                     vec![
//                         (
//                             String::from("Some"),
//                             Some(ast::TypeExpression::UniversalType(String::from("a")))
//                         ),
//                         (String::from("None"), None)
//                     ]
//                 ),
//                 ast::Statement::TypedLet(
//                     String::from("x"),
//                     ast::TypeExpression::DeclaredType(
//                         String::from("Option"),
//                         vec![ast::TypeExpression::IntType]
//                     ),
//                     vec![],
//                     ast::Expression::Block(
//                         vec![ast::Statement::UntypedLet(
//                             vec![String::from("y")],
//                             ast::Expression::IntegerLiteral(4)
//                         )],
//                         Box::new(ast::Expression::TypeVariant(
//                             String::from("Some"),
//                             Box::new(ast::Expression::Identifier(String::from("y")))
//                         ))
//                     )
//                 )
//             ]
//     );
//     // Err
//     assert!(parser
//         .parse(
//             "
//         let x = {
//             type Option 'a = Some with 'a | None;
//             let y = 4;
//             Some with y
//         };
//     "
//         )
//         .is_err());
//     assert!(parser.parse("let x = 4 let y = 5").is_err());
//     assert!(parser.parse("let x = ylet z = 4").is_err());
// }
