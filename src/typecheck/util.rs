/// Utility functions to help with type checking

pub fn next_typevar_name(k: &mut usize) -> String {
    *k += 1;
    return format!("τ{}", *k);
}


