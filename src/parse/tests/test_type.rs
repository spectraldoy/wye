use super::*;
use crate::types::structure::{Flex, Structure};
use crate::types::Type::*;
use std::collections::BTreeMap;

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
                None,
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

    assert!(parser.parse("'a").unwrap() == Poly("a".to_string(), Option::None));

    assert!(parser.parse("'_yusdf").unwrap() == Poly("_yusdf".to_string(), Option::None));
    assert!(parser.parse("'aAbB").unwrap() == Poly("aAbB".to_string(), Option::None));
    assert!(parser.parse("'a_1").unwrap() == Poly("a_1".to_string(), Option::None));
    assert!(parser.parse("'Type").unwrap() == Poly("Type".to_string(), Option::None));
    assert!(parser.parse("'___type").unwrap() == Poly("___type".to_string(), Option::None));

    // Bounded types
    assert!(
        parser.parse("Bounded'a").unwrap() == Poly("a".to_string(), Some("Bounded".to_string()))
    );
    assert!(parser.parse("__r9'a").unwrap() == Poly("a".to_string(), Some("__r9".to_string())));
    assert!(parser.parse("95'a").is_err());

    assert!(parser.parse("' tee").is_err());
    assert!(parser.parse("bound 'a").is_err());
    assert!(parser.parse(";df").is_err());
    assert!(parser.parse("'hello'").is_err());
    assert!(parser.parse("''").is_err());
    assert!(parser.parse("'95x").is_err());
}

#[test]
fn test_parse_type_identifier() {
    let parser = grammar::TypeParser::new();

    assert!(parser.parse("X").unwrap() == TypeId("X".to_string(), vec![]));
    assert!(
        parser.parse("'a Y").unwrap()
            == TypeId("Y".to_string(), vec![Poly("a".to_string(), Option::None)])
    );
    assert!(
        parser.parse("'a 'b X").unwrap()
            == TypeId(
                "X".to_string(),
                vec![
                    Poly("a".to_string(), Option::None),
                    Poly("b".to_string(), Option::None),
                ]
            )
    );
    assert!(
        parser.parse("'a z'b X").unwrap()
            == TypeId(
                "X".to_string(),
                vec![
                    Poly("a".to_string(), Option::None),
                    Poly("b".to_string(), Some("z".to_string())),
                ]
            )
    );
    assert!(
        parser.parse("'a X 'b Y").unwrap()
            == TypeId(
                "Y".to_string(),
                vec![
                    Poly("a".to_string(), Option::None),
                    TypeId("X".to_string(), vec![]),
                    Poly("b".to_string(), Option::None)
                ]
            )
    );

    assert!(
        parser.parse("Num'a Matrix").unwrap()
            == TypeId(
                "Matrix".to_string(),
                vec![Poly("a".to_string(), Some("Num".to_string()))]
            )
    );

    assert!(
        parser.parse("Num'a Matrix Option").unwrap()
            == TypeId(
                "Option".to_string(),
                vec![
                    Poly("a".to_string(), Some("Num".to_string())),
                    TypeId("Matrix".to_string(), vec![])
                ]
            )
    );
    assert!(
        parser.parse("(Num'a Matrix) Option").unwrap()
            == TypeId(
                "Option".to_string(),
                vec![TypeId(
                    "Matrix".to_string(),
                    vec![Poly("a".to_string(), Some("Num".to_string()))]
                )]
            )
    );
    assert!(
        parser.parse("A B").unwrap()
            == TypeId("B".to_string(), vec![TypeId("A".to_string(), vec![])])
    );
    assert!(parser.parse("int X").unwrap() == TypeId("X".to_string(), vec![Int]));
    assert!(
        parser.parse("Tree float X").unwrap()
            == TypeId(
                "X".to_string(),
                vec![TypeId("Tree".to_string(), vec![]), Float]
            )
    );
    assert!(
        parser.parse("((float Tree) Tree) X").unwrap()
            == TypeId(
                "X".to_string(),
                vec![TypeId(
                    "Tree".to_string(),
                    vec![TypeId("Tree".to_string(), vec![Float])]
                )]
            )
    );
    assert!(parser.parse("(int) X").unwrap() == TypeId("X".to_string(), vec![Int]));
    assert!(
        parser.parse("[int] Matrix").unwrap()
            == TypeId("Matrix".to_string(), vec![List(Box::new(Int))])
    );
    assert!(
        parser.parse("([int], float, none) X").unwrap()
            == TypeId(
                "X".to_string(),
                vec![Tuple(vec![List(Box::new(Int)), Float, None])]
            )
    );

    assert!(parser.parse("X [int]").is_err());
    assert!(parser.parse("Y 'a").is_err());
    assert!(parser.parse("'a 9").is_err());
    assert!(parser.parse("92__'a X").is_err());
    assert!(parser.parse("\"hi\" Option").is_err());
    assert!(parser.parse("(X Y").is_err());
    assert!(parser.parse("int float").is_err());
    assert!(parser.parse("int [y]").is_err());
    assert!(parser.parse("X with int").is_err());
}

// TODO: convert these tests to tests on typed let = nothing expressions

#[test]
fn test_parse_function_type() {
    let parser = grammar::TypeParser::new();

    assert!(parser.parse("int -> float").unwrap() == Function(Box::new(Int), Box::new(Float),));
    assert!(
        parser.parse("X -> Y").unwrap()
            == Function(
                Box::new(TypeId("X".to_string(), vec![])),
                Box::new(TypeId("Y".to_string(), vec![]))
            )
    );
    assert!(
        parser.parse("int -> none -> string").unwrap()
            == Function(
                Box::new(Int),
                Box::new(Function(Box::new(None), Box::new(String),))
            )
    );
    assert!(
        parser.parse("'a Tree -> int").unwrap()
            == Function(
                Box::new(TypeId(
                    "Tree".to_string(),
                    vec![Poly("a".to_string(), Option::None)]
                )),
                Box::new(Int)
            )
    );
    assert!(parser.parse("(int) -> (float)").unwrap() == Function(Box::new(Int), Box::new(Float),));
    assert!(parser.parse("(int -> float)").unwrap() == Function(Box::new(Int), Box::new(Float),));
    assert!(
        parser.parse("(int -> float) -> string").unwrap()
            == Function(
                Box::new(Function(Box::new(Int), Box::new(Float),)),
                Box::new(String)
            )
    );
    assert!(
        parser.parse("int->   float   ->string ->Option").unwrap()
            == Function(
                Box::new(Int),
                Box::new(Function(
                    Box::new(Float),
                    Box::new(Function(
                        Box::new(String),
                        Box::new(TypeId("Option".to_string(), vec![])),
                    ))
                ))
            )
    );
    assert!(
        parser.parse("'a -> 'b -> c").unwrap()
            == Function(
                Box::new(Poly("a".to_string(), Option::None)),
                Box::new(Function(
                    Box::new(Poly("b".to_string(), Option::None)),
                    Box::new(TypeId("c".to_string(), vec![])),
                ))
            )
    );
    assert!(
        parser.parse("int -> (float -> string) -> none").unwrap()
            == Function(
                Box::new(Int),
                Box::new(Function(
                    Box::new(Function(Box::new(Float), Box::new(String),)),
                    Box::new(None),
                ))
            )
    );
    assert!(
        parser.parse("int -> Num'a Option -> string").unwrap()
            == Function(
                Box::new(Int),
                Box::new(Function(
                    Box::new(TypeId(
                        "Option".to_string(),
                        vec![Poly("a".to_string(), Some("Num".to_string()))]
                    )),
                    Box::new(String)
                ))
            )
    );
    assert!(
        parser.parse("'a -> ('a -> 'a) -> 'a").unwrap()
            == Function(
                Box::new(Poly("a".to_string(), Option::None)),
                Box::new(Function(
                    Box::new(Function(
                        Box::new(Poly("a".to_string(), Option::None)),
                        Box::new(Poly("a".to_string(), Option::None)),
                    )),
                    Box::new(Poly("a".to_string(), Option::None))
                ))
            )
    );

    assert!(parser.parse("int (->) float").is_err());
    assert!(parser.parse("int -> 7").is_err());
    assert!(parser.parse("int -> string ->").is_err());
    assert!(parser.parse("int - > float").is_err());
    assert!(parser.parse("(a) -> (4)").is_err());
    assert!(parser.parse("Option 'a -> int").is_err());
    assert!(parser.parse("(x: int) -> (y: float)").is_err());
}

#[test]
fn test_parse_record_type() {
    let parser = grammar::TypeParser::new();

    assert!(
        parser.parse("{a: int}").unwrap()
            == Record(Structure {
                methods: BTreeMap::new(),
                values: BTreeMap::from([("a".to_string(), Int)]),
                flex: Flex::Permissive,
            })
    );
    assert!(
        parser.parse("{a: float, }").unwrap()
            == Record(Structure {
                methods: BTreeMap::new(),
                values: BTreeMap::from([("a".to_string(), Float)]),
                flex: Flex::Permissive,
            })
    );

    let optional_ending_comma_expected = Record(Structure {
        methods: BTreeMap::new(),
        values: BTreeMap::from([
            ("a".to_string(), Int),
            (
                "b".to_string(),
                TypeId(
                    "Option".to_string(),
                    vec![Poly("a".to_string(), Option::None)],
                ),
            ),
            ("c".to_string(), List(Box::new(Int))),
        ]),
        flex: Flex::Permissive,
    });
    assert!(
        parser
            .parse(
                "{
        a: int,
        b: 'a Option,
        c: [int]
    }"
            )
            .unwrap()
            == optional_ending_comma_expected
    );
    assert!(
        parser
            .parse(
                "{
        a: int,
        b: 'a Option,
        c: [int],
    }"
            )
            .unwrap()
            == optional_ending_comma_expected
    );

    assert!(parser
        .parse(
            "{
        a: int b: float
    }"
        )
        .is_err());
    assert!(
        parser
            .parse(
                "{|
        mem1: float,
        mem2: ({
            a: Three,
            method four: int
        }),
        mem4: [int],
        mem3: {| y: (int, float, none, { method u: int }) |}
    |}"
            )
            .unwrap()
            == Record(Structure {
                methods: BTreeMap::new(),
                values: BTreeMap::from([
                    ("mem1".to_string(), Float),
                    (
                        "mem2".to_string(),
                        Record(Structure {
                            methods: BTreeMap::from([("four".to_string(), Int)]),
                            values: BTreeMap::from([(
                                "a".to_string(),
                                TypeId("Three".to_string(), vec![])
                            )]),
                            flex: Flex::Permissive
                        })
                    ),
                    ("mem4".to_string(), List(Box::new(Int))),
                    (
                        "mem3".to_string(),
                        Record(Structure {
                            methods: BTreeMap::new(),
                            values: BTreeMap::from([(
                                "y".to_string(),
                                Tuple(vec![
                                    Int,
                                    Float,
                                    None,
                                    Record(Structure {
                                        methods: BTreeMap::from([("u".to_string(), Int)]),
                                        values: BTreeMap::new(),
                                        flex: Flex::Permissive,
                                    })
                                ]),
                            )]),
                            flex: Flex::Exact,
                        })
                    )
                ]),
                flex: Flex::Exact,
            })
    );
    assert!(
        parser
            .parse(
                "{|
        method a: int,
        method b: int -> int,
        u: float -> { u: string }
    |}"
            )
            .unwrap()
            == Record(Structure {
                methods: BTreeMap::from([
                    ("a".to_string(), Int),
                    ("b".to_string(), Function(Box::new(Int), Box::new(Int))),
                ]),
                values: BTreeMap::from([(
                    "u".to_string(),
                    Function(
                        Box::new(Float),
                        Box::new(Record(Structure {
                            methods: BTreeMap::new(),
                            values: BTreeMap::from([("u".to_string(), String)]),
                            flex: Flex::Permissive,
                        }))
                    )
                )]),
                flex: Flex::Exact,
            })
    );

    assert!(parser.parse("{4: int}").is_err());
    assert!(parser.parse("{}").is_err());
    assert!(parser.parse("{||}").is_err());
    assert!(parser.parse("{a: [4]}").is_err());
    assert!(parser.parse("{one: 2}").is_err());
    assert!(parser.parse("{int: a}").is_err());
    assert!(parser.parse("a: int").is_err());
    assert!(parser.parse("{a: int},").is_err());
    assert!(parser.parse("{int}").is_err());
    assert!(parser.parse("{a: int, a: float}").is_err());
}
