/// Utility functions for type inference
use super::Type;
use std::collections::HashMap;

/// Apply a substitution set to a type
pub fn apply_subst_type(subst: &HashMap<usize, Type>, typ: &Type) -> Type {
    match typ {
        // Unification with bounds is slightly harder. Always need to pick
        // The more specific bound. What if there are multiple bounds?
        // Then variables should have a set of bounds, right?
        Type::Variable(num, _) => {
            if subst.contains_key(num) {
                subst.get(num).unwrap().clone()
            } else {
                typ.clone()
            }
        }
        Type::TypeId(name, type_args) => {
            Type::TypeId(name.clone(), apply_subst_type_vec(subst, type_args))
        }
        Type::List(t) => Type::List(Box::new(apply_subst_type(subst, t))),
        Type::Tuple(elem_types) => Type::Tuple(apply_subst_type_vec(subst, elem_types)),
        Type::Function(arg, ret) => Type::Function(
            Box::new(apply_subst_type(subst, arg)),
            Box::new(apply_subst_type(subst, ret)),
        ),
        // most things here are TODO!
        _ => typ.clone(),
    }
}

/// Apply a substitution set
fn apply_subst_type_vec(subst: &HashMap<usize, Type>, type_vec: &Vec<Type>) -> Vec<Type> {
    type_vec
        .iter()
        .map(|typ| apply_subst_type(subst, typ))
        .collect()
}

/// Apply a substitution to another substitution. TODO(WYE-5): describe what this actually means
fn apply_subst_subst(
    subst_to_apply: &HashMap<usize, Type>,
    subst_applied_to: &HashMap<usize, Type>,
) -> HashMap<usize, Type> {
    // Apply subst_to_apply to every element of subst_applied_to
    let output = subst_applied_to
        .iter()
        .map(|(varnum, typ)| (*varnum, apply_subst_type(subst_to_apply, typ)))
        .collect();

    output
}

/// TODO: documentation
pub fn compose_substs(
    subst_to_apply: &HashMap<usize, Type>,
    subst_applied_to: &HashMap<usize, Type>,
) -> HashMap<usize, Type> {
    // apply subst_to_apply to subst_applied_to then fill in with the rest of
    // subst_to_apply that was not in subst_applied_to
    let mut output = apply_subst_subst(subst_to_apply, subst_applied_to);
    output.extend(subst_to_apply.clone());
    output
}

pub fn unify(typ1: &Type, typ2: &Type, cur_subst: &mut HashMap<usize, Type>) -> Result<(), String> {
    // TODO: get rid of the returns and use expression syntax
    match (typ1, typ2) {
        (Type::None, Type::None)
        | (Type::Int, Type::Int)
        | (Type::Float, Type::Float)
        | (Type::String, Type::String) => {}
        (Type::List(t1), Type::List(t2)) => unify(t1, t2, cur_subst)?,
        (Type::Function(f1_arg, f1_ret), Type::Function(f2_arg, f2_ret)) => {
            unify(f1_arg, f2_arg, cur_subst)?;
            unify(f1_ret, f2_ret, cur_subst)?
        }
        (Type::Variable(num1, _), Type::Variable(num2, _)) => {
            cur_subst.insert(*num1, Type::Variable(*num2, None));
        }
        (Type::Variable(num, _), t) | (t, Type::Variable(num, _)) => {
            cur_subst.insert(*num, t.clone());
        }
        // Actually there are still TODOs here
        _ => {
            return Err("Unify ahh!".to_string());
        }
    }

    Ok(())
}
