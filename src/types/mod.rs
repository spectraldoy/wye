pub mod check;
mod infer;
pub(crate) mod structure;

#[cfg(test)]
mod tests;

// In the Ocaml compiler, function types are
// a -> b
// but function expressions are vecs
// TODO: labeled, omittable func args
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    // Literal types
    None,
    Int,
    Float,
    String,
    // (name, type-params). Could be enum or struct, this is not known at parse-time
    TypeId(String, Vec<Type>),
    List(Box<Type>),
    Tuple(Vec<Type>),
    // { method? <id>: <type> } or {| method? <id?: type |}
    Record(structure::Structure),
    // a -> (b -> (...))
    // hold argument type, return type
    // TODO: argument label
    Function(Box<Type>, Box<Type>),
    // Type variable during inference.
    Variable(usize),
    // Identifier for polymorphic type and optional interface bound.
    // Maybe this needs to be a structural bound as well?
    Poly(String, Option<String>),
    // Type of module that can be opened to unwrap a namespace
    Module,
}

// None, Int, Float, String, enums, structs, lists, functions, these
// all have interfaces known statically from predefined methods and impld interfaces
// same with records
// It's variables and polymorphic types for which we don't know anything
// BIG TODO: having structures be passed around with variables means that field access
// no longer implies object or record type
// - i think that's a problem which is solved by modules or namespaced functions

// x signature -> protocol? interf? trait? proto? concept?
// TODO: separate out the structure from the types - it's not just variables
// that should have a structure being passed around of methods available on
// them
// by the way, isn't that structure important for translation to IR as well
// What if the way this was done, was signatures and their compiled code
// were basically defined in one central place, and that was automatically
// ingested by the TypeContext - and the IR machine eventually - so that I
// don't have to inject manually the typings of different things?
// We could have one centralized place src/builtin that defines both the IR
// and the type information for each of the builtin things

/// Utility function for collecting a sequence of types meant to represent
/// the type signature of a function, into a Type::Function.
pub fn collect_functype(types: &[Type]) -> Result<Type, String> {
    if types.len() == 0 {
        return Err("At least 2 types are required to construct a function".to_string());
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
