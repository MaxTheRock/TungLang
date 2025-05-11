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

    let path = std::path::Path::new(&args.file);
    if path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("tung")) != Some(true) {
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
            // Let the caller / shell know something went wrong.
            return Err(miette::miette!("Error parsing program: {}", e));
        }
    };

    // Use the original parsed for execute_program, then print_ast with parsed.clone()
    execute_program(parsed, &mut variables);
    println!("\n--- AST ---");
    // Only clone for pretty-printing, not for execution
    print_ast(&parsed.clone(), 0);

    Ok(())
}
