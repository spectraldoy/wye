use super::*;

#[test]
fn test_parse_identifiers() {
    let parser = grammar::WyeProgramParser::new();
    assert!(parser.parse("let x = 4;").is_ok());
}