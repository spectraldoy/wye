use super::util::OptionBox;
/// This file describes the Abstract Syntax Tree for Wye. A Wye program is, at
/// base, a sequence of Wye statements. At present, there are only 6 allowed Wye
/// statements:
/// - Expressions
/// - Enum declarations
/// - Struct declarations
/// - Interface declarations
/// - Interface implementations
/// - Main
///
/// Expressions evaluate to values of a particular type. Variables in methods or
/// let statements may be annotated with types in order to aid the type checker.
/// It is useful to describe these types in an abstract syntax within the AST.
use ordered_float::OrderedFloat;

// TODO: documentation

pub enum Statement {
    Expression(Expression),
    // enum <Id> = <variant> (| <variant>)*
    // variants have optional fields.
    EnumDeclaration {
        name: String,
        variants: Vec<(String, Option<Type>)>,
    },
    // struct <Id> { <Id>: type,+ }
    StructDeclaration {
        name: String,
        members: Vec<(String, Type)>,
    },
    // interface <Id> (requires <Id>+)? { (method|val): type|methodimpl }
    InterfaceDeclaration {
        name: String,
        requires: Vec<String>,
        // Optional method implementation
        methods: Vec<(String, Option<Expression>)>,
        values: Vec<(String, Type)>,
    },
    // impl <Id>: <Id> { (AttrSet|MethodImpl)+ }
    Implementation {
        for_struct: String,
        implemented_interface: String,
        attr_sets: Vec<AttrSet>,
        // id, arguments, expression
        method_impls: Vec<(String, Vec<String>, Expression)>,
    },
    // Main is just an expression in Wye
    Main(Expression),
}

/// All expressions describe some kind of computation that evaluates to a value,
/// which can be stored in a variable, or used in further expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Nothing,
    IntLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    List(Vec<Expression>),
    Tuple(Vec<Expression>),
    // { <id>: value, ..., <id>: value }
    Record(Vec<(String, Expression)>),
    // Reference a variable or function from the environment.
    Identifier(String),
    BuiltinBinaryOp(BuiltinBinaryOp),
    // Print a value to stdout
    Print,
    // Print a value to stderr
    Error,
    // <Enum Name> . <Variant Name> with Field
    EnumVariant {
        enum_name: String,
        variant: String,
        field: OptionBox<Expression>,
    },
    // <Id>.<Id>: could be enum or struct
    Projection(String, String),
    // <Id>#<Id>
    MethodAccess(String, String),
    // <Expr> <Expr>
    FuncApplication(Box<Expression>, Box<Expression>),
    // match <Expr> { <Pat> => <Expr>, ... , <Pat> => <Expr> }
    MatchConstruct {
        matchand: Box<Expression>,
        arms: Vec<(Pattern, Expression)>,
    },
    // \ <identifier>* -> Expr
    Lambda {
        args: Vec<String>,
        expr: Box<Expression>,
    },
    // Evaluate an expression and store in a variable of a type
    // let <id> (arguments & type-annotation) = <expression>
    Let {
        var_name: String,
        args: Vec<String>,
        var_type: Type,
        expr: Box<Expression>,
    },
    // Polymorphic let-in construct
    // let <id> (arguments & type-annotation) = <expr> in <expr>
    LetIn {
        var_name: String,
        args: Vec<String>,
        var_type: Type,
        value_expr: Box<Expression>,
        in_expr: Box<Expression>,
    },
    // Change the value of a variable. This is only allowed in struct methods.
    // The set expression evaluates to nothing.
    Set(AttrSet),
    // <attrset> in <expr>
    SetIn(AttrSet, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // Literal types
    None,
    Int,
    Float,
    String,
    Enum(String),
    List(Box<Type>),
    Tuple(Vec<Type>),
    // <StructId>? { method? <id>: <type> }
    Record(Option<String>, Vec<(String, Type, InterfaceMemberType)>),
    // Identifier for polymorphic type and optional interface bound.
    Polymorphic(String, Option<String>),
    Function(Box<Type>, Box<Type>),
    // Type to be inferred
    Hole,
}

/// There are certain reserved tokens used to denote builtin binary operations,
/// which are supported only between values of applicable types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDiv,

    Lt,
    Gt,
    Leq,
    Geq,
    Eq,
    Neq,

    Cons,
}

/// Structs can contain either values or methods.
pub enum InterfaceMemberType {
    Value,
    Method,
}

/// set <Id>.<Id> = Expr
/// This is an expression that is only allowed in method implementations.
struct AttrSet {
    entity: String,
    attribute: String,
    new_expr: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Wildcard,
    IntLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    Identifier(String),
    // TypeId (with Identifier)?
    TypeVariant(String, Box<Pattern>),
    // x :: xs
    ListConstruction(String, String),
    Union(Vec<Pattern>),
    Complement(Box<Pattern>),
    EmptyList,
    List(Vec<Pattern>),
    Tuple(Vec<Pattern>),
    // <pat> if <guard_expr> - take this only if pat matches and guard(pat) is true
    Guarded(Box<Pattern>, Expression),
    // case <e1> => <e2>; e1 must evaluate to boolean
    Case(Expression, Expression),
}
