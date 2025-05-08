mod interpreter;
mod parser;
mod value;

use std::env;
use std::fs;
use std::process;

use crate::interpreter::Interpreter;
use crate::parser::{Rule, TungParser};
use pest::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.tung>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    match fs::read_to_string(file_path) {
        Ok(content) => {
            // Try with the file rule which is defined in your grammar
            match TungParser::parse(Rule::file, &content) {
                Ok(parsed) => {
                    // Initialize interpreter and run the program
                    let mut interpreter: Interpreter = Interpreter::new();
                    interpreter.interpret(parsed);
                }
                Err(err) => {
                    eprintln!("Error parsing file: {}", err);
                    eprintln!("Make sure your file follows the Tung language syntax.");

                    // For debugging: try with other potential entry rules if available
                    for rule in [Rule::statement, Rule::expression].iter() {
                        println!("Attempting to parse with rule {:?}...", rule);
                        if let Ok(_) = TungParser::parse(*rule, &content) {
                            println!("Parsing succeeded with rule {:?}", rule);
                            break;
                        }
                    }

                    process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    }
}
