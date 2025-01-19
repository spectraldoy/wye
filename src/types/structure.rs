//! Functionality for working with structural typing.
use super::infer;
use super::Type;
use std::collections::HashMap;

/// Flexibility of the structural type, for reusing functionality between
/// nominal and structural records
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flex {
    Permissive,
    CollectExact,
    Exact,
}

// TODO: use Structure everywhere, and unify nominal and structure records
/// The basic definition of a structural type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Structure {
    // TODO: polytype variables
    pub methods: HashMap<String, Type>,
    pub values: HashMap<String, Type>,
    pub flex: Flex,
}

impl Structure {
    pub fn new(methods: HashMap<String, Type>, values: HashMap<String, Type>, flex: Flex) -> Self {
        Self {
            methods,
            values,
            flex,
        }
    }

    pub fn from_values<I>(values: I) -> Self
    where
        I: IntoIterator<Item = (String, Type)>,
    {
        Self {
            methods: HashMap::new(),
            values: HashMap::from(values),
            flex,
        }
    }

    pub fn empty() -> Self {
        Self {
            methods: HashMap::new(),
            values: HashMap::new(),
            flex: Flex::Permissive,
        }
    }

    // /// Struct bounds can be satisfied by nominal or structural records
    // pub fn is_satisfied_by(&self, typ: &Type) -> bool {
    //     match typ {
    //         Type::Record(Structure {
    //             methods: typ_methods,
    //             values: typ_values,
    //             flex,
    //         }) => {
    //             let mut cur_subst = HashMap::new();
    //             // self's methods amd values should be a subset of typ's
    //             for (self_attr, typ_attr) in
    //                 [(&self.methods, typ_methods), (&self.values, typ_values)]
    //             {
    //                 for (field_name, field_type) in self_attr {
    //                     if !typ_attr.contains_key(field_name) {
    //                         return false;
    //                     }

    //                     let unif_res = infer::unify(
    //                         field_type,
    //                         self_attr.get(field_name).unwrap(),
    //                         &mut cur_subst,
    //                     );
    //                     if unif_res.is_err() {
    //                         return false;
    //                     }
    //                 }

    //                 if self.flex == Flex::Exact {

    //                 }
    //             }

    //             true
    //         }
    //         _ => false,
    //     }
    // }
}

// TODO: nominal record types can only be set equal to exact match
// nominal records.
