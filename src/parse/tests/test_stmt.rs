use super::ast::PolytypeVar;
use super::ast::Statement::EnumDecl;
use super::ast::Type;
use super::span::UnSpan;
use super::*;
use std::collections::HashMap;

fn parse_enum_decl(parser: &grammar::StatementParser, inp: &'static str) -> ast::Statement {
    let out = parser.parse(inp).unwrap().unspanned();
    if let EnumDecl {
        name: _,
        type_args: _,
        variants: _,
        span: _,
    } = out
    {
        out
    } else {
        panic!("Input is not an enum declaration.")
    }
}

#[test]
fn test_parse_enum_decl() {
    let parser = grammar::StatementParser::new();

    assert!(
        parse_enum_decl(&parser, "enum A = B")
            == EnumDecl {
                name: ("A".to_string(), None),
                type_args: vec![],
                variants: vec![("B".to_string(), None, None)],
                span: None,
            }
    );

    assert!(
        parse_enum_decl(&parser, "enum bool = false | true")
            == EnumDecl {
                name: ("bool".to_string(), None),
                type_args: vec![],
                variants: vec![
                    ("false".to_string(), None, None),
                    ("true".to_string(), None, None),
                ],
                span: None,
            }
    );

    assert!(
        parse_enum_decl(&parser, "enum A = B | C | D")
            == EnumDecl {
                name: ("A".to_string(), None),
                type_args: vec![],
                variants: vec![
                    ("B".to_string(), None, None),
                    ("C".to_string(), None, None),
                    ("D".to_string(), None, None)
                ],
                span: None,
            }
    );

    assert!(
        parse_enum_decl(
            &parser,
            "enum Thing = Var1
    | Var2 with int
    | Var3 with float
    | Var4 with [string]"
        ) == EnumDecl {
            name: ("Thing".to_string(), None),
            type_args: vec![],
            variants: vec![
                ("Var1".to_string(), None, None),
                ("Var2".to_string(), Some(Type::Int), None),
                ("Var3".to_string(), Some(Type::Float), None),
                (
                    "Var4".to_string(),
                    Some(Type::List(Box::new(Type::String))),
                    None
                ),
            ],
            span: None,
        }
    );

    assert!(
        parse_enum_decl(&parser, "enum 'a bin_tree = Leaf | Node with { val: 'a, left: 'a bin_tree, right: 'a bin_tree }")
        == EnumDecl {
            name: ("bin_tree".to_string(), None),
            type_args: vec![PolytypeVar {
                name: "a".to_string(),
                bound: None,
                span: None,
            }],
            variants: vec![
                ("Leaf".to_string(), None, None),
                ("Node".to_string(), Some(Type::StructRecord {
                    methods: HashMap::new(),
                    values: HashMap::from([
                        ("val".to_string(), Type::Poly("a".to_string(), None)),
                        ("left".to_string(), Type::TypeId("bin_tree".to_string(), vec![Type::Poly("a".to_string(), None)])),
                        ("right".to_string(), Type::TypeId("bin_tree".to_string(), vec![Type::Poly("a".to_string(), None)])),
                    ]),
                }), None)
            ],
            span: None,
        }
    );

    assert!(
        parse_enum_decl(&parser, "enum 'a Option = None | Some with 'a")
            == EnumDecl {
                name: ("Option".to_string(), None),
                type_args: vec![PolytypeVar {
                    name: "a".to_string(),
                    bound: None,
                    span: None,
                },],
                variants: vec![
                    ("None".to_string(), None, None),
                    (
                        "Some".to_string(),
                        Some(Type::Poly("a".to_string(), None)),
                        None
                    ),
                ],
                span: None,
            }
    );

    assert!(
        parse_enum_decl(
            &parser,
            "enum X = Y with int Option | Z with Num'a 'b Matrix"
        ) == EnumDecl {
            name: ("X".to_string(), None),
            type_args: vec![],
            variants: vec![
                (
                    "Y".to_string(),
                    Some(Type::TypeId("Option".to_string(), vec![Type::Int])),
                    None
                ),
                (
                    "Z".to_string(),
                    Some(Type::TypeId(
                        "Matrix".to_string(),
                        vec![
                            Type::Poly("a".to_string(), Some("Num".to_string())),
                            Type::Poly("b".to_string(), None),
                        ]
                    )),
                    None
                )
            ],
            span: None,
        }
    );

    assert!(parser.parse("enum A = B, C").is_err());
    assert!(parser.parse("enum A = B with [int, string]").is_err());
    assert!(parser.parse("enum A = B with (int, string)").is_ok());
    assert!(parser.parse("A = B | C").is_err());
    assert!(parser.parse("A | B | C").is_err());
    assert!(parser.parse("enum A = B C | D").is_err());
    assert!(parser.parse("enum X = A B C").is_err());
    assert!(parser.parse("enum A = ").is_err());
    assert!(parser.parse("enum A B = C").is_err());
    assert!(parser.parse("enum 'a -> 'b Option = Some | None").is_err());
    assert!(parser.parse("enumbool = Thing").is_err());
    assert!(parser.parse("let enum A = B | C").is_err());
}
