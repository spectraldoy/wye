use super::span::{unspanned_seq, GetSpan, OptionSpan, Span, UnSpan};
use super::util::OptionBox;
use crate::types::Type;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

/// This file describes the Abstract Syntax Tree for Wye. A Wye program is, at
/// base, a sequence of Wye statements. At present, there are only 5 allowed Wye
/// statements:
/// - Expressions
/// - Enum declarations
/// - Struct declarations
/// - Interface declarations
/// - Interface implementations
///
/// Expressions evaluate to values of a particular type. Variables in methods or
/// let statements may be annotated with types in order to aid the type checker.
/// It is useful to describe these types in an abstract syntax within the AST.

// TODO(WYE-5): documentation
// TODO: and keyword for group bindings / mutual recursion

pub type Program = Vec<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    // enum <Id> <polytype var>* = <variant> (| <variant>)*
    // variants have optional fields.
    // These, unlike Expressions, are not recursive structures
    EnumDecl {
        name: (String, OptionSpan),
        type_args: Vec<PolytypeVar>,
        variants: Vec<(String, Option<Type>, OptionSpan)>,
        span: OptionSpan,
    },
    // struct <Id> <polytype var>* { <Id>: type,+ }
    StructDecl {
        name: (String, OptionSpan),
        type_args: Vec<PolytypeVar>,
        members: Vec<(String, Type, OptionSpan)>,
    },
    // sig <Id> <polytype var>* (requires (<Id> | (<Id> <polytype var>+))+)? { (method|val): type|methodimpl }
    InterfaceDecl {
        name: (String, OptionSpan),
        type_args: Vec<PolytypeVar>,
        requires: Vec<(String, OptionSpan, Vec<PolytypeVar>)>,
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
        for_struct: (String, OptionSpan, Vec<PolytypeVar>),
        impl_interface: Option<(String, OptionSpan, Vec<PolytypeVar>)>,
        attr_sets: Vec<AttrSet>,
        // id, arguments, expression
        method_impls: Vec<VarWithValue>,
    },
}

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
    StructRecord(HashMap<String, (Expression, OptionSpan)>, OptionSpan),
    // {| <id>: value, ..., <id>: value |}
    NominalRecord(HashMap<String, (Expression, OptionSpan)>, OptionSpan),
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
    // <Id>.<Id>: could be enum or struct or attribute
    Projection(Box<Expression>, String, OptionSpan),
    // <Expr>#<Id>
    MethodAccess(Box<Expression>, String, OptionSpan),
    // <Expr> args
    FuncApplication(Box<Expression>, Vec<Expression>, OptionSpan),
    // <Expr> arg1=e1 arg2=e2 ...
    NamedArgsFuncApp(
        Box<Expression>,
        Vec<(String, Expression, OptionSpan)>,
        OptionSpan,
    ),
    // match <Expr> { <Pat> => <Expr>, ... , <Pat> => <Expr> }
    Match {
        matchand: Box<Expression>,
        arms: Vec<(Pattern, Expression)>,
        span: OptionSpan,
    },
    // \ <identifier> -> Expr
    Lambda {
        arg: String,
        expr: Box<Expression>,
        span: OptionSpan,
    },
    // Evaluate an expression and store in a variable of a type
    // Poly let-in construct
    // let <id> (arguments & type-annotation) = <expression> (in thing)?
    Let(VarWithValue, OptionBox<Expression>, OptionSpan),
    // Change the value of a variable. This is only allowed in object methods.
    // The set expression evaluates to nothing.
    // thing <- expr
    Set(AttrSet, OptionSpan),
}

/// Reserved tokens used to denote builtin binary operations, which are
/// supported only between values of applicable types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    /// Arithmetic operations.
    Add,
    Sub,
    Mult,
    Div,
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
    pub args: Vec<(String, OptionSpan)>,
    pub rec: bool,
    pub expr: Box<Expression>,
}

/// set <Id>.<Id> = Expr
/// This is an expression that is only allowed in method implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttrSet {
    pub entity: (String, OptionSpan),
    pub attr: (String, OptionSpan),
    pub new_expr: Box<Expression>,
}

/// (bound)?'name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolytypeVar {
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
    // TypeId (with Pattern)
    TypeVariant(String, OptionBox<Pattern>, OptionSpan),
    // x :: xs
    ListCons(String, String, OptionSpan),
    EmptyList(OptionSpan),
    Union(Vec<Pattern>, OptionSpan),
    Complement(Box<Pattern>, OptionSpan),
    List(Vec<Pattern>, OptionSpan),
    Tuple(Vec<Pattern>, OptionSpan),
    // <pat> if <guard_expr> - take this only if pat matches and guard(pat) is true
    Guarded {
        pattern: Box<Pattern>,
        guard: Expression,
        span: OptionSpan,
    },
    // case <e>; e must evaluate to boolean
    Case(Expression, OptionSpan),
}

impl UnSpan for Statement {
    fn unspanned(&self) -> Self {
        match self {
            Self::Expression(e) => Self::Expression(e.unspanned()),
            Self::EnumDecl {
                name,
                type_args,
                variants,
                span: _,
            } => Self::EnumDecl {
                name: (name.0.clone(), None),
                type_args: unspanned_seq(&type_args),
                variants: variants
                    .iter()
                    .map(|v| (v.0.clone(), v.1.clone(), None))
                    .collect(),
                span: None,
            },
            Self::StructDecl {
                name,
                type_args,
                members,
            } => Self::StructDecl {
                name: (name.0.clone(), None),
                type_args: unspanned_seq(&type_args),
                members: members
                    .iter()
                    .map(|m| (m.0.clone(), m.1.clone(), None))
                    .collect(),
            },
            Self::InterfaceDecl {
                name,
                type_args,
                requires,
                impl_methods,
                spec_methods,
                values,
            } => Self::InterfaceDecl {
                name: (name.0.clone(), None),
                type_args: unspanned_seq(&type_args),
                requires: requires
                    .iter()
                    .map(|r| (r.0.clone(), None, unspanned_seq(&r.2)))
                    .collect(),
                impl_methods: unspanned_seq(&impl_methods),
                spec_methods: spec_methods
                    .iter()
                    .map(|m| (m.0.clone(), m.1.clone(), None))
                    .collect(),
                values: values
                    .iter()
                    .map(|v| (v.0.clone(), v.1.clone(), None))
                    .collect(),
            },
            Self::InterfaceImpl {
                for_struct,
                impl_interface,
                attr_sets,
                method_impls,
            } => Self::InterfaceImpl {
                for_struct: (for_struct.0.clone(), None, unspanned_seq(&for_struct.2)),
                impl_interface: match impl_interface {
                    Some((name, _, type_args)) => {
                        Some((name.clone(), None, unspanned_seq(&type_args)))
                    }
                    None => None,
                },
                attr_sets: unspanned_seq(&attr_sets),
                method_impls: unspanned_seq(&method_impls),
            },
        }
    }
}

impl UnSpan for Expression {
    fn unspanned(&self) -> Self {
        match &self {
            Self::Nothing(_) => Self::Nothing(None),
            Self::IntLiteral(i, _) => Self::IntLiteral(*i, None),
            Self::FloatLiteral(f, _) => Self::FloatLiteral(*f, None),
            Self::StringLiteral(s, _) => Self::StringLiteral(s.clone(), None),
            Self::List(lst, _) => Self::List(unspanned_seq(&lst), None),
            Self::Tuple(tup, _) => Self::Tuple(unspanned_seq(&tup), None),
            Self::StructRecord(rec, _) => Self::StructRecord(
                rec.iter()
                    .map(|r| (r.0.clone(), (r.1 .0.unspanned(), None)))
                    .collect(),
                None,
            ),
            Self::NominalRecord(rec, _) => Self::NominalRecord(
                rec.iter()
                    .map(|r| (r.0.clone(), (r.1 .0.unspanned(), None)))
                    .collect(),
                None,
            ),
            Self::Identifier(id, _) => Self::Identifier(id.clone(), None),
            Self::BinaryOp(binop, _) => Self::BinaryOp(binop.clone(), None),
            Self::Print(_) => Self::Print(None),
            Self::Fail(_) => Self::Fail(None),
            Self::EnumVariant {
                enum_id,
                variant,
                field,
                span: _,
            } => Self::EnumVariant {
                enum_id: enum_id.clone(),
                variant: variant.clone(),
                field: Box::new(field.unspanned()),
                span: None,
            },
            Self::Projection(e, id, _) => {
                Self::Projection(Box::new(e.unspanned()), id.clone(), None)
            }
            Self::MethodAccess(e, id, _) => {
                Self::MethodAccess(Box::new(e.unspanned()), id.clone(), None)
            }
            Self::FuncApplication(e, args, _) => {
                Self::FuncApplication(Box::new(e.unspanned()), unspanned_seq(&args), None)
            }
            Self::NamedArgsFuncApp(e, args, _) => Self::NamedArgsFuncApp(
                Box::new(e.unspanned()),
                args.iter()
                    .map(|(name, arg, _)| (name.clone(), arg.unspanned(), None))
                    .collect(),
                None,
            ),
            Self::Match {
                matchand,
                arms,
                span: _,
            } => Self::Match {
                matchand: Box::new(matchand.unspanned()),
                arms: arms
                    .iter()
                    .map(|(p, e)| (p.unspanned(), e.unspanned()))
                    .collect(),
                span: None,
            },
            Self::Lambda { arg, expr, span: _ } => Self::Lambda {
                arg: arg.clone(),
                expr: Box::new(expr.unspanned()),
                span: None,
            },
            Self::Let(v, e, _) => {
                let new_e = if let Some(box_expr) = e {
                    Some(Box::new(box_expr.unspanned()))
                } else {
                    None
                };
                Self::Let(v.unspanned(), new_e, None)
            }
            Self::Set(a, _) => Self::Set(a.unspanned(), None),
        }
    }
}

impl UnSpan for VarWithValue {
    fn unspanned(&self) -> Self {
        Self {
            name: (self.name.0.clone(), None),
            args: self.args.iter().map(|v| (v.0.clone(), None)).collect(),
            rec: self.rec,
            expr: Box::new(self.expr.unspanned()),
        }
    }
}

impl UnSpan for AttrSet {
    fn unspanned(&self) -> Self {
        Self {
            entity: (self.entity.0.clone(), None),
            attr: (self.attr.0.clone(), None),
            new_expr: Box::new(self.new_expr.unspanned()),
        }
    }
}

impl UnSpan for PolytypeVar {
    fn unspanned(&self) -> Self {
        Self {
            name: self.name.clone(),
            bound: self.bound.clone(),
            span: None,
        }
    }
}

impl UnSpan for Pattern {
    fn unspanned(&self) -> Self {
        match &self {
            Self::Wildcard(_) => Self::Wildcard(None),
            Self::IntLiteral(i, _) => Self::IntLiteral(*i, None),
            Self::FloatLiteral(f, _) => Self::FloatLiteral(*f, None),
            Self::StringLiteral(s, _) => Self::StringLiteral(s.clone(), None),
            Self::Identifier(id, _) => Self::Identifier(id.clone(), None),
            Self::TypeVariant(tid, field, _) => {
                let field = match &field {
                    Some(p) => Some(Box::new(p.unspanned())),
                    _ => None,
                };
                Self::TypeVariant(tid.clone(), field, None)
            }
            Self::ListCons(s1, s2, _) => Self::ListCons(s1.clone(), s2.clone(), None),
            Self::EmptyList(_) => Self::EmptyList(None),
            Self::Union(pv, _) => Self::Union(unspanned_seq(&pv), None),
            Self::Complement(p, _) => Self::Complement(Box::new(p.unspanned()), None),
            Self::List(pv, _) => Self::List(unspanned_seq(&pv), None),
            Self::Tuple(pv, _) => Self::Tuple(unspanned_seq(&pv), None),
            Self::Guarded {
                pattern,
                guard,
                span: _,
            } => Self::Guarded {
                pattern: Box::new(pattern.unspanned()),
                guard: guard.unspanned(),
                span: None,
            },
            Self::Case(e, _) => Self::Case(e.unspanned(), None),
        }
    }
}

#[cfg(not(test))]
impl GetSpan for Expression {
    fn get_span(&self) -> Span {
        match &self {
            Self::Nothing(s) => s.as_ref().unwrap().clone(),
            Self::IntLiteral(_, s) => s.as_ref().unwrap().clone(),
            Self::FloatLiteral(_, s) => s.as_ref().unwrap().clone(),
            Self::StringLiteral(_, s) => s.as_ref().unwrap().clone(),
            Self::List(_, s) => s.as_ref().unwrap().clone(),
            Self::Tuple(_, s) => s.as_ref().unwrap().clone(),
            Self::StructRecord(_, s) => s.as_ref().unwrap().clone(),
            Self::NominalRecord(_, s) => s.as_ref().unwrap().clone(),
            Self::Identifier(_, s) => s.as_ref().unwrap().clone(),
            Self::BinaryOp(_, s) => s.as_ref().unwrap().clone(),
            Self::Print(s) => s.as_ref().unwrap().clone(),
            Self::Fail(s) => s.as_ref().unwrap().clone(),
            Self::EnumVariant { span, .. } => span.as_ref().unwrap().clone(),
            Self::Projection(_, _, s) => s.as_ref().unwrap().clone(),
            Self::MethodAccess(_, _, s) => s.as_ref().unwrap().clone(),
            Self::FuncApplication(_, _, s) => s.as_ref().unwrap().clone(),
            Self::NamedArgsFuncApp(_, _, s) => s.as_ref().unwrap().clone(),
            Self::Match { span, .. } => span.as_ref().unwrap().clone(),
            Self::Lambda { span, .. } => span.as_ref().unwrap().clone(),
            Self::Let(_, _, s) => s.as_ref().unwrap().clone(),
            Self::Set(_, s) => s.as_ref().unwrap().clone(),
        }
    }
}

// TODO: What are the easter eggs in the grammar?
// null == none
// nether makes the program execute bottom to top
// terminate halts the program and prints a link to "I'll be back?"
