use ordered_float::OrderedFloat;

pub type Identifier<'a> = &'a str;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    IntegerLiteral(i64),
    FloatLiteral(OrderedFloat<f64>),
    StringLiteral(String),
    List(Vec<Expression<'a>>),
    Tuple(Vec<Expression<'a>>),
    Identifier(Identifier<'a>),
    // { Statement* Expression }
    Block(Vec<Statement<'a>>, Box<Expression<'a>>),
    // <TypeId> ( with <Field> )?
    TypeVariant(Identifier<'a>, Box<Expression<'a>>),
    // <Expr> <Expr>
    FuncApplication(Box<Expression<'a>>, Box<Expression<'a>>),
    // <Expr> <Op> <Expr>
    BinaryOperation(Box<Expression<'a>>, Operation, Box<Expression<'a>>),
    // match <Expr> { <Pat> => <Expr>; ... ; <Pat> => <Expr> }
    MatchConstruct(Box<Expression<'a>>, Vec<Pattern<'a>>, Vec<Expression<'a>>),
    // if <Expr> then <Expr> else <Expr>
    Conditional(Box<Expression<'a>>, Box<Expression<'a>>, Box<Expression<'a>>),
    // print <Expr>
    PrintExpression(Box<Expression<'a>>),
    // error <ErrorMessage>
    ErrorExpression(String)
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
    FunctionType(Vec<TypeExpression<'a>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<'a> {
    Wildcard,
    ExactMatch(Box<Expression<'a>>),
    // <head identifier>, <tail identifier>
    ListConstruction(Identifier<'a>, Identifier<'a>),
    // <VariantTypeId> ( with <field id> )?
    TypeVariant(Identifier<'a>, Option<Identifier<'a>>),
    PatternList(Vec<Pattern<'a>>),
    PatternTuple(Vec<Pattern<'a>>),
}

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