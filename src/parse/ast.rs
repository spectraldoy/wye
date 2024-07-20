use ordered_float::OrderedFloat;

pub type Identifier<'a> = &'a str;

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
pub enum Expression<'a> {
    IntegerLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    List(Vec<Expression<'a>>),
    Tuple(Vec<Expression<'a>>),
    Identifier(Identifier<'a>),
    BuiltinOp(Operation),
    Print,
    Error,
    // <TypeId> ( with <Field> )?
    TypeVariant(Identifier<'a>, Box<Expression<'a>>),
    // <Expr> <Expr>
    FuncApplication(Box<Expression<'a>>, Box<Expression<'a>>),
    // match <Expr> { <Pat> => <Expr>; ... ; <Pat> => <Expr> }
    MatchConstruct(Box<Expression<'a>>, Vec<(Pattern<'a>, Expression<'a>)>),
    // anonymous function created by \ <id*> -> expr
    Lambda(Vec<Identifier<'a>>, Box<Expression<'a>>),
    // if <Expr> then <Expr> else <Expr>
    Conditional(Box<Expression<'a>>, Box<Expression<'a>>, Box<Expression<'a>>),
    // { Statement* Expression }
    Block(Vec<Statement<'a>>, Box<Expression<'a>>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<'a> {
    Wildcard,
    IntegerLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    Identifier(Identifier<'a>),
    // TypeId (with Identifier)?
    TypeVariant(Identifier<'a>, Box<Pattern<'a>>),
    // x :: xs
    ListConstruction(Identifier<'a>, Identifier<'a>),
    // Union
    Union(Vec<Pattern<'a>>),
    Complement(Box<Pattern<'a>>),
    EmptyList,
    List(Vec<Pattern<'a>>),
    Tuple(Vec<Pattern<'a>>),
    // pat if guard - take this only if pat matches and guard(pat) is true
    Guarded(Box<Pattern<'a>>, Expression<'a>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpression<'a> {
    IntType,
    FloatType,
    StringType,
    DeclaredType(Identifier<'a>, Vec<TypeExpression<'a>>),
    ListType(Box<TypeExpression<'a>>),
    TupleType(Vec<TypeExpression<'a>>),
    TypeVariable(Identifier<'a>),
    FunctionType(Box<TypeExpression<'a>>, Box<TypeExpression<'a>>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    // let <Id> <Id>* = <Expr> ;
    UntypedLet(Identifier<'a>, Vec<Identifier<'a>>, Box<Expression<'a>>),
    // let <Id> ( <Id>: <Type> -> )* <Type> = <Expr>
    // translated to [(identifier, TypeExpr)] Expression
    TypedLet(Vec<(Identifier<'a>, TypeExpression<'a>)>, Box<Expression<'a>>),
    // type <TypeId> <TypeArgs>? = ( <TypeId> (with <Type?)? )+
    // translated into TypeId, TypeVars, VariantNames, VariantFields
    TypeDeclaration(Identifier<'a>, Vec<Identifier<'a>>, Vec<(Identifier<'a>, Option<TypeExpression<'a>>)>)
}