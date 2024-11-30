fn main() {
    // This is necessary for our grammar to be specified in LALRPOP syntax.
    lalrpop::process_root().unwrap();
}
