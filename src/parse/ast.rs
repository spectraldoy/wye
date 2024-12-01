use super::util::OptionBox;
use ordered_float::OrderedFloat;
use super::span::Spanned;

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

// TODO: documentation

// It is unfortunate but recursive children might have to be spanned
// this is simply part of the grammar I guess, and it is that way because
// we have to know the possibile erroneousness of children
// The spans thing is legitimately a design problem

pub type Program = Vec<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Expression(Spanned<Expression>),
    // enum <Id> <polytype var>* = <variant> (| <variant>)*
    // variants have optional fields.
    // These, unlike Expressions, are not recursive structures
    EnumDecl {
        name: Spanned<String>,
        polytype_vars: Vec<Spanned<PolytypeDecl>>,
        variants: Vec<Spanned<(String, Option<Type>)>>,
    },
    // struct <Id> <polytype var>* { <Id>: type,+ }
    StructDecl {
        name: Spanned<String>,
        polytype_vars: Vec<Spanned<PolytypeDecl>>,
        members: Vec<Spanned<(String, Type)>>,
    },
    // interface <Id> <polytype var>* (requires (<Id> | (<Id> <polytype var>+))+)? { (method|val): type|methodimpl }
    InterfaceDecl {
        name: Spanned<String>,
        polytype_vars: Vec<Spanned<PolytypeDecl>>,
        requires: Vec<Spanned<(String, Vec<PolytypeDecl>)>>,
        // Implemented methods
        // name, args, output type, expression
        impl_methods: Vec<VarWithValue>,
        // Unimplemented methods
        spec_methods: Vec<Spanned<(String, Type)>>,
        values: Vec<Spanned<(String, Type)>>,
    },
    // impl <Id>: <Id> { (AttrSet|MethodImpl)+ }
    InterfaceImpl {
        // name and type vars
        for_struct: (Spanned<String>, Vec<Spanned<PolytypeDecl>>),
        implemented_interface: Option<(Spanned<String>, Vec<Spanned<PolytypeDecl>>)>,
        attr_sets: Vec<AttrSet>,
        // id, arguments, expression
        method_impls: Vec<VarWithValue>,
    },
    // Main is just an expression in Wye
    Main(Spanned<Expression>),
    // Erroneous statement
    Error(&'static str),
}

/// All expressions describe some kind of computation that evaluates to a value,
/// which can be stored in a variable, or used in further expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Nothing,
    IntLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    List(Vec<Spanned<Expression>>),
    Tuple(Vec<Spanned<Expression>>),
    // { <id>: value, ..., <id>: value }
    Record(Vec<Spanned<Expression>>),
    // Reference a variable or function from the environment.
    Identifier(String),
    BinaryOp(BinaryOp),
    // Print a value to stdout
    Print,
    // Print a value to stderr and exit.
    Fail,
    // <Enum Name> . <Variant Name> with Field
    EnumVariant {
        enum_name: String,
        variant: String,
        field: OptionBox<Spanned<Expression>>,
    },
    // <Id>.<Id>: could be enum or struct
    Projection(String, String),
    // <Id>#<Id>
    MethodAccess(String, String),
    // <Expr> <Expr>
    FuncApplication(Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    // match <Expr> { <Pat> => <Expr>, ... , <Pat> => <Expr> }
    MatchConstruct {
        matchand: Box<Spanned<Expression>>,
        arms: Vec<(Spanned<Pattern>, Spanned<Expression>)>,
    },
    // \ <identifier>* -> Expr
    Lambda {
        args: Vec<String>,
        expr: Box<Spanned<Expression>>,
    },
    // Evaluate an expression and store in a variable of a type
    // let <id> (arguments & type-annotation) = <expression>
    Let(VarWithValue),
    // Polymorphic let-in construct
    // let <id> (arguments & type-annotation) = <expr> in <expr>
    LetIn(VarWithValue, Box<Spanned<Expression>>),
    // Change the value of a variable. This is only allowed in struct methods.
    // The set expression evaluates to nothing.
    Set(AttrSet),
    // <attrset> in <expr>
    SetIn(AttrSet, Box<Spanned<Expression>>),
    // Erroneous expression
    Error(&'static str),
}

/// TODO
/// f a b = c
/// f a: int -> b: string -> int = c
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarWithValue {
    pub name: Spanned<String>,
    pub args: Spanned<Vec<(String, Type)>>,
    pub out_type: Spanned<Type>,
    pub expr: Box<Spanned<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // Literal types
    None,
    Int,
    Float,
    String,
    Enum { name: String, polytype_vars: Vec<PolytypeDecl> },
    List(Box<Type>),
    Tuple(Vec<Type>),
    // <StructId>? { method? <id>: <type> }
    Record {
        struct_id: Option<String>,
        methods: Vec<(String, Type)>,
        values: Vec<(String, Type)>,
    },
    // Identifier for polymorphic type and optional interface bound.
    Polymorphic { name: String, bound: Option<String> },
    Function(Box<Type>, Box<Type>),
    // Type to be inferred
    Hole,
    // Erroneous type
    Error(&'static str),
}

/// Reserved tokens used to denote builtin binary operations, which are
/// supported only between values of applicable types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    /// Arithmetic operations.
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDiv,

    /// Comparators.
    Lt,
    Gt,
    Leq,
    Geq,
    Eq,
    Neq,

    /// List construction.
    Cons,
}

/// Interfaces contain values and methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceMemberType {
    Value,
    Method,
    Error(&'static str),
}

/// set <Id>.<Id> = Expr
/// This is an expression that is only allowed in method implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttrSet {
    pub entity: Spanned<String>,
    pub attribute: Spanned<String>,
    pub new_expr: Box<Spanned<Expression>>,
}

/// ' (bound)? name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolytypeDecl {
    pub name: String,
    pub bound: Option<String>,
}

/// Pattern matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Wildcard,
    IntLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    Identifier(String),
    // TypeId (with Identifier)?
    TypeVariant(String, Option<String>),
    // x :: xs
    ListConstruction(String, String),
    Union(Vec<Spanned<Pattern>>),
    Complement(Box<Spanned<Pattern>>),
    EmptyList,
    List(Vec<Spanned<Pattern>>),
    Tuple(Vec<Spanned<Pattern>>),
    // <pat> if <guard_expr> - take this only if pat matches and guard(pat) is true
    Guarded(Box<Spanned<Pattern>>, Spanned<Expression>),
    // case <e>; e must evaluate to boolean
    Case(Spanned<Expression>),
    // Error, for reporting.
    Error(&'static str),
}

// TODO: What are the easter eggs in the grammar?
// null == none