use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    IntegerLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    List(Vec<Expression>),
    Tuple(Vec<Expression>),
    Identifier(String),
    BuiltinOp(Operation),
    Print,
    Error,
    // <TypeId> ( with <Field> )?
    TypeVariant(String, Box<Expression>),
    // <Expr> <Expr>
    FuncApplication(Box<Expression>, Box<Expression>),
    // match <Expr> { <Pat> => <Expr>; ... ; <Pat> => <Expr> }
    MatchConstruct(Box<Expression>, Vec<(Pattern, Expression)>),
    // anonymous function created by \ <id*> -> expr
    Lambda(Vec<String>, Box<Expression>),
    // { Statement* Expression }
    Block(Vec<Statement>, Box<Expression>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Wildcard,
    IntegerLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    Identifier(String),
    // TypeId (with Identifier)?
    TypeVariant(String, Box<Pattern>),
    // x :: xs
    ListConstruction(String, String),
    // Union
    Union(Vec<Pattern>),
    Complement(Box<Pattern>),
    EmptyList,
    List(Vec<Pattern>),
    Tuple(Vec<Pattern>),
    // pat if guard - take this only if pat matches and guard(pat) is true
    Guarded(Box<Pattern>, Expression)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpression {
    IntType,
    FloatType,
    StringType,
    DeclaredType(String, Vec<TypeExpression>),
    ListType(Box<TypeExpression>),
    TupleType(Vec<TypeExpression>),
    TypeVariable(String),
    FunctionType(Box<TypeExpression>, Box<TypeExpression>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    // let <Id> <Id>* = <Expr> ;
    UntypedLet(Vec<String>, Expression),
    // let <Id> ( <Id>: <Type> -> )* <Type> = <Expr>
    // translated to [(identifier, TypeExpr)] Expression
    TypedLet(String, TypeExpression, Vec<(String, TypeExpression)>, Expression),
    // type <TypeId> <TypeArgs>? = ( <TypeId> (with <Type?)? )+
    // translated into TypeId, TypeVars, VariantNames, VariantFields
    TypeDeclaration(String, Vec<String>, Vec<(String, Option<TypeExpression>)>)
}