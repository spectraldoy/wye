/// Utility functions for type inference
use super::Type;
use super::check::TypeContext;
use std::collections::HashMap;

/// Apply a substitution set to a type
pub fn apply_subst_type(subst: &HashMap<u128, Type>, typ: &Type) -> Type {
    match typ {
        Type::Variable(num) => {
            if subst.contains_key(num) {
                subst.get(num).unwrap().clone()
            } else {
                typ.clone()
            }
        },
        Type::TypeId(name, type_args) => {
            Type::TypeId(name.clone(), apply_subst_type_vec(subst, type_args))
        },
        Type::List(t) => Type::List(apply_subst_type(subst, t)),
        Type::Tuple(elem_types) => Type::Tuple(apply_subst_type_vec(subst, elem_types)),
        Type::Function(arg_types) => Type::Function(apply_subst_type_vec(subst, arg_types)),
        _ => typ.clone()
    }
}

/// Apply a substitution set
fn apply_subst_type_vec(subst: &HashMap<u128, Type>, type_vec: &Vec<Type>) -> Vec<Type> {
    type_vec.iter().map(|typ| apply_subst_type(subst, typ)).collect()
}

/// Apply a substitution to another substitution. TODO(WYE-5): describe what this actually means
fn apply_subst_subst(subst_to_apply: &HashMap<u128, Type>, subst_applied_to: &HashMap<u128, Type>) -> HashMap<u128, Type> {
    let output = subst_applied_to.clone();
    for (varnum, _) in &subst_applied_to {
        if subst_to_apply.contains_key(&varnum) {
            output.insert(varnum, subst_to_apply.get(&varnum).unwrap());
        }
    }

    output
}

/// TODO: documentation
pub fn compose_substs(subst1: &HashMap<u128, Type>, subst2: &HashMap<u128, Type>) -> HashMap<u128, Type> {
    // apply subst1 to subst2 then update with subst1
    let mut output = apply_subst_subst(subst1, subst2);
    output.extend(subst_to_apply);
    output
}

pub fn unify(typ1: &Type, typ2: &Type, cur_subst: &mut Hashmap<u128, Type>) -> Result<(), String> {
    // TODO: get rid of the returns and use expression syntax
    match (typ1, typ2) {
        (Type::None, Type::None)
        | (Type::Int, Type::Int)
        | (Type::Float, Type::Float)
        | (Type::String, Type::String) => {},
        (Type::List(t1), Type::List(t2)) => unify(t1, t2, cur_subst),
        (Type::Function(arg_types1), Type::Function(arg_types2)) => {
            if arg_types1.len() != arg_types2.len() {
                return Err("function ahh!".to_string());
            }
            for (arg_type1, arg_type2) in arg_types1.iter().zip(arg_types2.iter()) {
                let res = unify(arg_type1, arg_type2, cur_subst);
                if res.is_err() {
                    return res;
                }
            }
        }
        (Type::Variable(num1), Type::Variable(num2)) => cur_subst.insert(num1, Type::Variable(num2)),
        (Type::Variable(num), t) | (t, Type::Variable(num)) => cur_subst.insert(num, t.clone()),
        _ => todo!()
    }

    Ok(())
}
