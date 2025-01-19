//! Functionality for working with structural typing.
use super::Type;
use std::collections::BTreeMap;

/// Flexibility of the structural type, for reusing functionality between
/// nominal and structural records
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Flex {
    Permissive,
    CollectExact,
    Exact,
}

// TODO: use Structure everywhere, and unify nominal and structure records
/// The basic definition of a structural type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Structure {
    // TODO: polytype variables
    pub methods: BTreeMap<String, Type>,
    pub values: BTreeMap<String, Type>,
    pub flex: Flex,
}

impl Structure {
    pub fn new(
        methods: BTreeMap<String, Type>,
        values: BTreeMap<String, Type>,
        flex: Flex,
    ) -> Self {
        Self {
            methods,
            values,
            flex,
        }
    }

    pub fn empty() -> Self {
        Self {
            methods: BTreeMap::new(),
            values: BTreeMap::new(),
            flex: Flex::Permissive,
        }
    }
}

// TODO: nominal record types can only be set equal to exact match
// nominal records.
