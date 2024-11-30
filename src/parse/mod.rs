use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod span;
mod util;
lalrpop_mod!(pub grammar, "/parse/grammar.rs");

#[cfg(test)]
mod tests;
