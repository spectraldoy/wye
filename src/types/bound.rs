use super::infer;
use super::Type;
/// Functionality for defining or checking bounds on generic or structural types.
use std::collections::{BTreeSet, HashMap};

// A bound satisfied by a set of types
pub struct SetBound {
    // Types are not hashable because they contain HashMaps, which are not hashable
    // as they are unordered. We use a BTreeSet here as a compromise.
    pub members: BTreeSet<Type>,
}

impl SetBound {
    pub fn from<I>(member_types: I) -> Self
    where
        I: IntoIterator<Item = Type>,
    {
        Self {
            members: member_types.into_iter().collect(),
        }
    }
}

/// A bound satisfied by structural subtypes.
pub struct StructBound {
    // Map from polytype variable name to optional bound
    // TODO: polytype variables
    methods: HashMap<String, Type>,
    values: HashMap<String, Type>,
}

impl StructBound {
    pub fn new(methods: HashMap<String, Type>, values: HashMap<String, Type>) -> Self {
        Self { methods, values }
    }

    /// Struct bounds can be satisfied by nominal or structural records
    pub fn is_satisfied_by(&self, typ: &Type) -> bool {
        match typ {
            Type::StructRecord {
                methods: typ_methods,
                values: typ_values,
            }
            | Type::NominalRecord {
                methods: typ_methods,
                values: typ_values,
            } => {
                let mut cur_subst = HashMap::new();
                // self's methods amd values should be a subset of typ's
                for (self_attr, typ_attr) in
                    [(&self.methods, typ_methods), (&self.values, typ_values)]
                {
                    for (field_name, field_type) in self_attr {
                        if !typ_attr.contains_key(field_name) {
                            return false;
                        }

                        let unif_res = infer::unify(
                            field_type,
                            self_attr.get(field_name).unwrap(),
                            &mut cur_subst,
                        );
                        if unif_res.is_err() {
                            return false;
                        }
                    }
                }

                true
            }
            _ => false,
        }
    }
}

// TODO: nominal record types can only be set equal to exact match
// nominal records.
