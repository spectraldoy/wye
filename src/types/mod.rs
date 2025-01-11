use std::collections::HashMap;

pub mod check;
mod infer;

#[cfg(test)]
mod tests;

// In the Ocaml compiler, function types are
// a -> b
// but function expressions are vecs
// TODO: labeled, omittable func args
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
    // a -> (b -> (...))
    Function(Box<Type>, Box<Type>),
    // Type variable during inference
    // The argument is the "name" of the variable
    // TODO: type variables can be constrained by bounds
    Variable(usize),
}

/// Utility function for collecting a sequence of types meant to represent
/// the type signature of a function, into a Type::Function.
pub fn collect_functype(types: &[Type]) -> Result<Type, &'static str> {
    if types.len() == 0 {
        return Err("At least 2 types are required to construct a function");
    }
    if types.len() == 1 {
        return Ok(types[0].clone());
    }

    let output_type = collect_functype(&types[1..])?;
    Ok(Type::Function(
        Box::new(types[0].clone()),
        Box::new(output_type),
    ))
}
