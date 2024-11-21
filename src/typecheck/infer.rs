/// Methods for type inference
use crate::parse::ast::TypeExpression;
use std::collections::HashMap;

/// Substitute an inferable type with for_type in type expression in_type
/// We expect by this time that all type expressions in the program have been checked
/// to use the correct number of type arguments for declared types and to only use
/// type variables that are present in the scope of the type
pub fn subst_inftype_in_type_expr(inftypename: &String, for_type: &TypeExpression, in_type: &TypeExpression) -> TypeExpression {
    match in_type {
        TypeExpression::DeclaredType(typename, type_args) => {
            let mapped_args: Vec<TypeExpression> = type_args
                .iter()
                .map(|ta| subst_inftype_in_type_expr(inftypename, for_type, ta))
                .collect::<_>();
            TypeExpression::DeclaredType(typename.clone(), mapped_args)
        },
        TypeExpression::ListType(te) => {
            TypeExpression::ListType(Box::new(subst_inftype_in_type_expr(inftypename, for_type, te)))
        },
        TypeExpression::TupleType(tup_types) => {
            let mapped_types: Vec<TypeExpression> = tup_types
                .iter()
                .map(|t| subst_inftype_in_type_expr(inftypename, for_type, t))
                .collect::<_>();
            TypeExpression::TupleType(mapped_types)
        },
        TypeExpression::FunctionType(te_func, te_arg) => {
            TypeExpression::FunctionType(
                Box::new(subst_inftype_in_type_expr(inftypename, for_type, te_func)),
                Box::new(subst_inftype_in_type_expr(inftypename, for_type, te_arg))
            )
        },
        // here we actually perform the substitution
        TypeExpression::InferableType(enc_inftypename) => {
            if enc_inftypename == inftypename {
                for_type.clone()
            } else {
                in_type.clone()
            }
        },
        // Any of the other types require no substitution
        _ => in_type.clone()
    }
}

/// A substitution is just a hashmap mapping String names of inferable types to TypeExpressions
/// to substitute them with. This function applies an entire substitution to a type expression
pub fn apply_subst_type_expr(
    substitution: &HashMap<String, TypeExpression>,
    type_expr: &TypeExpression,
) -> TypeExpression {
    let mut out = type_expr.clone();
    for (inftypename, subst_type) in substitution.iter() {
        out = subst_inftype_in_type_expr(inftypename, subst_type, &out);
    }
    out
}

/// To apply a substitution to a type environment, which is itself a hash map from names of
/// identifiers to the type expressions or guessed type expressions which they hold, is to
/// apply the substitution to each type expression mapped to by an identifier
pub fn apply_subst_type_env(
    substitution: &HashMap<String, TypeExpression>,
    type_env: &HashMap<String, TypeExpression>
) -> HashMap<String, TypeExpression> {
    type_env
        .iter()
        .map(|(id, typ)| (
            id.clone(),
            apply_subst_type_expr(substitution, typ)
        ))
        .collect::<_>()
}

/// TODO: a substitution should reflect sets of type expressions

/// apply substitution to a substitution. A substitution is a mapping from names of inferable
/// types to type expressions or sets of type expressions. When we apply substitution A to
/// substitution B we mean replacing all instances of { τ_k: te_k } in B with { τ_k: te_k2 }
/// where the latter substitution occurs in A. This process occurs in order to express
/// further type constraints on the variables in our program as we read more of the
/// program. However, in case te_k and te_k2 do not agree - that is, they cannot be reconciled
/// then this indicates that we have found two type constraints for a particular type
/// that cannot be solved. Thus, in such a case, we must return Err
pub fn apply_subst_subst(
    subst1: &HashMap<String, TypeExpression>,
    subst2: &HashMap<String, TypeExpression>
) -> Result<HashMap<String, TypeExpression>, String> {
    let mut out = subst2.clone();
    for (inftypename, te) in subst1.iter() {
        if subst2.contains_key(inftypename) {
            let other_te = subst2.get(inftypename).unwrap();
            let resolved_type = resolve_type_exprs(te, other_te);
            if let Err(e) = resolved_type {
                return Err(e);
            }
        } else {
            out.insert(inftypename.clone(), te.clone());
        }
    }

    Ok(out)
}

// TODO: universal types are better called GenericTypes
/// determine whether or not two type expressions can express the same type.
/// if so, return the resolved type and the substitutions necessary to make it happen
/// A substitution here is basically an assertion that two types are equal
/// This is actually the job of the unifier. It needs to know the context in order to do this properly
fn resolve_type_exprs(
    typ1: &TypeExpression,
    typ2: &TypeExpression
) -> Result<TypeExpression, String> {
    match (typ1, typ2) {
        (TypeExpression::IntType, _)
        | (TypeExpression::FloatType, _)
        | (TypeExpression::StringType, _) => {
            match typ2 {
                x if x == typ1 => Ok(typ1.clone()),
                // involves substitution of the universal type variable for this function
                // with the current type. So do the universal type variables also count
                // as types that can be substituted? There are two contexts here that we have
                // to consider: in the case of type checking a type constructor instantiated
                // with a universal type, then the universal type is indeed substituted
                // by the type of the argument. However, inferable types operate differently
                // in that the kinds of constraints that end up on universal types must
                // not contradict their universality
                TypeExpression::UniversalType(_) => Ok(typ1.clone()),
                _ => Err(String::from("bah"))
            }
        }
        _ => Err(String::from("unresolvable"))
    }
}