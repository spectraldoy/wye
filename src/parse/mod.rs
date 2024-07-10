use lalrpop_util::lalrpop_mod;

pub mod ast;
lalrpop_mod!(pub grammar, "/parse/grammar.rs");

#[cfg(test)]
mod tests;