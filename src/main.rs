mod value;
mod ast;
mod eval;

use clap::Parser as ClapParser;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fs;
use crate::value::Value;
use crate::ast::print_ast;
use crate::eval::execute_program;

#[derive(Parser)]
#[grammar = "tung.pest"]
pub struct TungParser;

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the TungLang source file
    #[arg(short, long)]
    pub file: String,
}

fn main() -> miette::Result<()> {
    let args: Args = Args::parse();

    if !args.file.ends_with(".tung") {
        return Err(miette::miette!("Error: Only .tung files are allowed."));
    }

    let program: String = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(e) => {
            return Err(miette::miette!("Error reading file {}: {}", args.file, e));
        }
    };

    let mut variables: HashMap<String, Value> = HashMap::new();

    let parsed = match TungParser::parse(crate::Rule::program, &program) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error parsing program: {}", e);
            return Ok(());
        }
    };

    execute_program(parsed.clone(), &mut variables);
    println!("\n--- AST ---");
    print_ast(&parsed, 0);

    Ok(())
}
