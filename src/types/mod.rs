use std::collections::HashMap;

mod check;
mod infer;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // Literal types
    None,
    Int,
    Float,
    String,
    // (name, type-params). Could be enum or struct, this is not known at parse-time
    TypeId(String, Vec<Type>),
    // Identifier for polymorphic type and optional interface bound.
    Poly(String, Option<String>),
    List(Box<Type>),
    Tuple(Vec<Type>),
    // { method? <id>: <type> }
    NominalRecord {
        methods: HashMap<String, Type>,
        values: HashMap<String, Type>,
    },
    StructRecord {
        methods: HashMap<String, Type>,
        values: HashMap<String, Type>,
    },
    // a -> b -> ...
    Function(Vec<Type>),
    // Type variable during inference
    // The argument is the "name" of the variable
    // TODO: type variables can be constrained by bounds
    Variable(u128),
}
