// These are slightly more restricted in the actually parsing functionality.
// See wye.lalrpop
pub type Identifier = String;
pub type TypeId = String;

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    // type <TypeId> <TypeArgs>? = ( <TypeId> (with <Type?)? )+
    // translated into TypeId, TypeVars, VariantNames, VariantFields
    TypeDeclaration(TypeId, Vec<Identifier>, Vec<TypeId>, Vec<Option<TypeExpression>>)
}

#[derive(Debug, Clone)]
pub enum LetStatement {
    // let <Id> <Id>* = <Expr>
    UntypedLet(Identifier, Vec<Identifier>, Box<Expression>),
    // let <Id> ( <Id>: <Type> -> )* <Type> = <Expr>
    // translated to lhs, typeof lhs, argnames, argtypes
    TypedLet(Identifier, Box<TypeExpression>, Vec<Identifier>, Vec<TypeExpression>, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    List(Vec<Expression>),
    Tuple(Vec<Expression>),
    Variable(Identifier),
    // <TypeId> ( with <Field> )?
    TypeVariant(TypeId, Option<Box<Expression>>),
    // <Expr> <Expr>
    FuncApplication(Box<Expression>, Box<Expression>),
    // match <Expr> { <Pat> => <Expr>; ... ; <Pat> => <Expr> }
    MatchConstruct(Box<Expression>, Vec<Pattern>, Vec<Expression>),
    // if <Expr> then <Expr> else <Expr>
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    // print <Expr>
    PrintExpression(Box<Expression>),
    // error <ErrorMessage>
    ErrorExpression(String)
}

#[derive(Debug, Clone)]
pub enum TypeExpression {
    IntType,
    FloatType,
    StringType,
    DeclaredType(TypeId, Vec<TypeExpression>),
    ListType(Box<TypeExpression>),
    TupleType(Vec<TypeExpression>),
    TypeVariable(Identifier),
    FunctionType(Vec<TypeExpression>),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    ExactMatch(Box<Expression>),
    // <head identifier>, <tail identifier>
    ListConstruction(Identifier, Identifier),
    // <VariantType> ( with <field id> )?
    TypeVariant(TypeId, Option<Identifier>),
    PatternList(Vec<Pattern>),
    PatternTuple(Vec<Pattern>),
}


