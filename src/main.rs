mod compiler;
mod interpreter;
mod parser;

use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file passed");
        return Ok(());
    }
    let source = fs::read_to_string(&args[1])?;
    let parsed_source = parser::parse(&source)?;
    let ast = parser::construct_ast(parsed_source);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.run(&ast);

    Ok(())
}
