use super::span::{unspanned_vec, OptionSpan, UnSpan};
use super::util::OptionBox;
use ordered_float::OrderedFloat;

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
        impl_interface: Option<(String, OptionSpan, Vec<PolytypeDecl>)>,
        attr_sets: Vec<AttrSet>,
        // id, arguments, expression
        method_impls: Vec<VarWithValue>,
    },
    // Main is just an expression in Wye
    Main(Expression),
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
    // <Expr> args
    FuncApplication(Box<Expression>, Vec<Expression>, OptionSpan),
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
    Polymorphic {
        name: String,
        bound: Option<String>,
    },
    // a -> b -> ...
    Function(Vec<Type>),
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
    pub attr: (String, OptionSpan),
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
                polytype_vars,
                variants,
            } => Self::EnumDecl {
                name: (name.0.clone(), None),
                polytype_vars: unspanned_vec(&polytype_vars),
                variants: variants
                    .iter()
                    .map(|v| (v.0.clone(), v.1.clone(), None))
                    .collect(),
            },
            Self::StructDecl {
                name,
                polytype_vars,
                members,
            } => Self::StructDecl {
                name: (name.0.clone(), None),
                polytype_vars: unspanned_vec(&polytype_vars),
                members: members
                    .iter()
                    .map(|m| (m.0.clone(), m.1.clone(), None))
                    .collect(),
            },
            Self::InterfaceDecl {
                name,
                polytype_vars,
                requires,
                impl_methods,
                spec_methods,
                values,
            } => Self::InterfaceDecl {
                name: (name.0.clone(), None),
                polytype_vars: unspanned_vec(&polytype_vars),
                requires: requires
                    .iter()
                    .map(|r| (r.0.clone(), None, unspanned_vec(&r.2)))
                    .collect(),
                impl_methods: unspanned_vec(&impl_methods),
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
                for_struct: (for_struct.0.clone(), None, unspanned_vec(&for_struct.2)),
                impl_interface: match impl_interface {
                    Some((name, _, polytype_vars)) => {
                        Some((name.clone(), None, unspanned_vec(&polytype_vars)))
                    }
                    None => None,
                },
                attr_sets: unspanned_vec(&attr_sets),
                method_impls: unspanned_vec(&method_impls),
            },
            Self::Main(e) => Self::Main(e.unspanned()),
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
            Self::List(lst, _) => Self::List(unspanned_vec(&lst), None),
            Self::Tuple(tup, _) => Self::Tuple(unspanned_vec(&tup), None),
            Self::Record(rec, _) => Self::Record(
                rec.iter()
                    .map(|r| (r.0.clone(), r.1.unspanned(), None))
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
            Self::Projection(id1, id2, _) => Self::Projection(id1.clone(), id2.clone(), None),
            Self::MethodAccess(id1, id2, _) => Self::MethodAccess(id1.clone(), id2.clone(), None),
            Self::FuncApplication(e1, args, _) => {
                Self::FuncApplication(Box::new(e1.unspanned()), unspanned_vec(&args), None)
            }
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
            Self::Let(v, _) => Self::Let(v.unspanned(), None),
            Self::LetIn(v, _, e) => Self::LetIn(v.unspanned(), None, Box::new(e.unspanned())),
            Self::Set(a, _) => Self::Set(a.unspanned(), None),
            Self::SetIn(a, _, e) => Self::SetIn(a.unspanned(), None, Box::new(e.unspanned())),
        }
    }
}

impl UnSpan for VarWithValue {
    fn unspanned(&self) -> Self {
        Self {
            name: (self.name.0.clone(), None),
            args: self
                .args
                .iter()
                .map(|v| (v.0.clone(), v.1.clone(), None))
                .collect(),
            out_type: (self.out_type.0.clone(), None),
            expr: (Box::new(self.expr.0.unspanned()), None),
        }
    }
}

impl UnSpan for AttrSet {
    fn unspanned(&self) -> Self {
        Self {
            entity: (self.entity.0.clone(), None),
            attr: (self.attr.0.clone(), None),
            new_expr: (Box::new(self.new_expr.0.unspanned()), None),
        }
    }
}

impl UnSpan for PolytypeDecl {
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
            Self::Union(pv, _) => Self::Union(unspanned_vec(&pv), None),
            Self::Complement(p, _) => Self::Complement(Box::new(p.unspanned()), None),
            Self::List(pv, _) => Self::List(unspanned_vec(&pv), None),
            Self::Tuple(pv, _) => Self::Tuple(unspanned_vec(&pv), None),
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

// TODO: What are the easter eggs in the grammar?
// null == none
// terminate halts the program and prints a link to "I'll be back?"
