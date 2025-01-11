use std::env;
use std::fs;
use std::path::Path;
use wye::parse::grammar;
use wye::types::check;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: cargo run <command> <path to wye file>");
        std::process::exit(1);
    }

    let wye_program =
        fs::read_to_string(Path::new(&args[2])).expect("Failed to read provided file");
    let action = &args[1];

    let parser: grammar::ProgramParser = grammar::ProgramParser::new();

    match action.as_str() {
        "parse" | "p" => {
            println!("{:?}", parser.parse(wye_program.as_str()));
        }
        "typecheck" | "tc" => {
            let parsed_prog = parser.parse(wye_program.as_str()).unwrap();
            println!("{:?}", check::type_check_program(&parsed_prog));
        }
        _ => {
            println!("Unknown or unimplemented language action: {}", action);
            std::process::exit(1);
        }
    }
}
