mod ast;
mod error;
mod interpreter;
mod parser;

use std::env;
use std::fs;
use std::process;

use crate::error::pest_error_with_source;
use interpreter::Interpreter;
use miette::{IntoDiagnostic, Result};
use parser::{parse_statement, ParserContext, TungLangParser};
use pest::Parser; // Import the Parser trait

fn main() -> Result<()> {
    // Set up miette's fancy reporter
    miette::set_hook(Box::new(|_| {
        let handler = miette::MietteHandlerOpts::new()
            .color(true)
            .unicode(true)
            .build();
        Box::new(handler)
    }))?;

    // Get file path from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.tung>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Validate file extension
    if !file_path.ends_with(".tung") {
        eprintln!("Error: File must have a .tung extension");
        process::exit(1);
    }

    // Read the file
    let source = fs::read_to_string(file_path).into_diagnostic()?;

    // Create parser context with source for error reporting
    let parser_ctx = ParserContext::new(source.clone());

    // Debugging output to trace parsing process
    println!("Parsing file: {}", file_path);
    println!("Source content:\n{}", source);

    // Parse the file
    let parse_result = TungLangParser::parse(parser::Rule::program, &source);

    // Debugging output for parse result
    match &parse_result {
        Ok(pairs) => println!("Parse result: {:?}", pairs),
        Err(e) => {
            println!("Parse error: {:?}", e);
            eprintln!("Error: Failed to parse the file. Please check the syntax.");
        }
    };

    // Handle the result
    match parse_result {
        Ok(pairs) => {
            println!("Successfully parsed {}:", file_path);

            let mut interpreter = Interpreter::new(source.clone());
            let mut statements = Vec::new();

            // Extract program pairs (skip the outer program rule)
            let program_pairs = pairs.into_iter().next().unwrap().into_inner();

            // Parse each statement
            for pair in program_pairs {
                if pair.as_rule() == parser::Rule::EOI {
                    continue;
                }

                let stmt = parse_statement(pair, &parser_ctx)?;
                statements.push(stmt);
            }

            // Execute the statements
            println!("\nProgram output:");
            for stmt in &statements {
                interpreter.execute(stmt)?;
            }

            Ok(())
        }
        Err(e) => {
            // Convert pest error to our fancy error type with source
            let fancy_error = pest_error_with_source(e, source);
            eprintln!("Error: {}", fancy_error);
            Err(fancy_error.into())
        }
    }
}
