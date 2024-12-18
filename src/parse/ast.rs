use ordered_float::OrderedFloat;
use super::span::OptionSpan;

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

// TODO(WYE-5): documentation

pub type Program = Vec<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    // enum <Id> <polytype var>* = <variant> (| <variant>)*
    // variants have optional fields.
    // These, unlike Expressions, are not recursive structures
    EnumDecl {
        name: (String, OptionSpan),
        polytype_vars: Vec<PolytypeDecl>,
        variants: Vec<(String, Option<Type>, OptionSpan)>,
    },
    // struct <Id> <polytype var>* { <Id>: type,+ }
    StructDecl {
        name: (String, OptionSpan),
        polytype_vars: Vec<PolytypeDecl>,
        members: Vec<(String, Type, OptionSpan)>,
    },
    // interface <Id> <polytype var>* (requires (<Id> | (<Id> <polytype var>+))+)? { (method|val): type|methodimpl }
    InterfaceDecl {
        name: (String, OptionSpan),
        polytype_vars: Vec<PolytypeDecl>,
        requires: Vec<(String, OptionSpan, Vec<PolytypeDecl>)>,
        // Implemented methods
        // name, args, output type, expression
        impl_methods: Vec<VarWithValue>,
        // Unimplemented methods
        spec_methods: Vec<(String, Type, OptionSpan)>,
        values: Vec<(String, Type, OptionSpan)>,
    },
    // impl <Id>: <Id> { (AttrSet|MethodImpl)+ }
    InterfaceImpl {
        // name and type vars
        for_struct: (String, OptionSpan, Vec<PolytypeDecl>),
        implemented_interface: Option<(String, OptionSpan, Vec<PolytypeDecl>)>,
        attr_sets: Vec<AttrSet>,
        // id, arguments, expression
        method_impls: Vec<VarWithValue>,
    },
    // Main is just an expression in Wye
    Main(Expression),
}


// You could make spans optional
// and then have a parser argument, depending on which we 
// collect spans or not
// you still have to have unspans right, where you replace
// all the spans with Nones
// and apart from that, you also need to have equality checks

/// Expressions describe some kind of computation that evaluates to a value,
/// which can be stored in a variable, or used in further expressions.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Nothing(OptionSpan),
    IntLiteral(i64, OptionSpan),
    FloatLiteral(OrderedFloat<f64>, OptionSpan),
    StringLiteral(String, OptionSpan),
    List(Vec<Expression>, OptionSpan),
    Tuple(Vec<Expression>, OptionSpan),
    // { <id>: value, ..., <id>: value }
    Record(Vec<(String, Expression, OptionSpan)>, OptionSpan),
    // Reference a variable or function from the environment.
    Identifier(String, OptionSpan),
    BinaryOp(BinaryOp, OptionSpan),
    // Print a value to stdout
    Print(OptionSpan),
    // Print a value to stderr and exit.
    Fail(OptionSpan),
    // <Enum Name> . <Variant Name> with Field
    EnumVariant {
        enum_id: String,
        variant: String,
        field: Box<Expression>,
        span: OptionSpan,
    },
    // <Id>.<Id>: could be enum or struct
    Projection(String, String, OptionSpan),
    // <Id>#<Id>
    MethodAccess(String, String, OptionSpan),
    // <Expr> <Expr>
    FuncApplication(Box<Expression>, Box<Expression>, OptionSpan),
    // match <Expr> { <Pat> => <Expr>, ... , <Pat> => <Expr> }
    MatchConstruct {
        matchand: Box<Expression>,
        arms: Vec<(Pattern, Expression)>,
        span: OptionSpan,
    },
    // \ <identifier>* -> Expr
    Lambda {
        args: Vec<(String, OptionSpan)>,
        expr: Box<Expression>,
        span: OptionSpan
    },
    // Evaluate an expression and store in a variable of a type
    // let <id> (arguments & type-annotation) = <expression>
    Let(VarWithValue, OptionSpan),
    // Polymorphic let-in construct
    // let <id> (arguments & type-annotation) = <expr> in <expr>
    LetIn(VarWithValue, OptionSpan, Box<Expression>),
    // Change the value of a variable. This is only allowed in struct methods.
    // The set expression evaluates to nothing.
    Set(AttrSet, OptionSpan),
    // <attrset> in <expr>
    SetIn(AttrSet, OptionSpan, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // Literal types
    None,
    Int,
    Float,
    String,
    // (name, polytypes). Could be enum or struct, this is not known at parse-time
    TypeId(String, Vec<PolytypeDecl>),
    List(Box<Type>),
    Tuple(Vec<Type>),
    // { method? <id>: <type> }
    Record {
        methods: Vec<(String, Type)>,
        values: Vec<(String, Type)>,
    },
    // Identifier for polymorphic type and optional interface bound.
    Polymorphic { name: String, bound: Option<String> },
    Function(Box<Type>, Box<Type>),
    // Type to be inferred
    Hole,
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

/// TODO(WYE-5): Documentation
/// f a b = c
/// f a: int -> b: string -> int = c
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarWithValue {
    pub name: (String, OptionSpan),
    pub args: Vec<(String, Type, OptionSpan)>,
    pub out_type: (Type, OptionSpan),
    pub expr: (Box<Expression>, OptionSpan),
}

/// set <Id>.<Id> = Expr
/// This is an expression that is only allowed in method implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttrSet {
    pub entity: (String, OptionSpan),
    pub attribute: (String, OptionSpan),
    pub new_expr: (Box<Expression>, OptionSpan),
}

/// ' (bound)? name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolytypeDecl {
    pub name: String,
    pub bound: Option<String>,
    pub span: OptionSpan,
}

/// Pattern matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Wildcard(OptionSpan),
    IntLiteral(i64, OptionSpan),
    FloatLiteral(OrderedFloat<f64>, OptionSpan),
    StringLiteral(String, OptionSpan),
    Identifier(String, OptionSpan),
    // TypeId (with Identifier)?
    TypeVariant(String, Option<String>, OptionSpan),
    // x :: xs
    ListConstruction(String, String, OptionSpan),
    EmptyList(OptionSpan),
    Union(Vec<Pattern>, OptionSpan),
    Complement(Box<Pattern>, OptionSpan),
    List(Vec<Pattern>, OptionSpan),
    Tuple(Vec<Pattern>, OptionSpan),
    // <pat> if <guard_expr> - take this only if pat matches and guard(pat) is true
    Guarded {
        pattern: Box<Pattern>,
        guard_expr: Expression,
        span: OptionSpan
    },
    // case <e>; e must evaluate to boolean
    Case(Expression, OptionSpan),
}

// TODO: What are the easter eggs in the grammar?
// null == none
// terminate halts the program and prints a link to "I'll be back?"
