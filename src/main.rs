use miette::{IntoDiagnostic, Result};
use pest::Parser;
use std::env;
use std::fs;
use std::process;

mod diagnostics;
mod interpreter;
mod keywords;
mod parser;
mod value;

use crate::diagnostics::TungError;
use crate::interpreter::Interpreter;
use crate::parser::{Rule, TungParser};

fn main() -> Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path> [keywords_config]", args[0]);
        process::exit(1);
    }

    // Load default keywords first
    if let Err(err) = keywords::load_default_keywords() {
        eprintln!("Warning: Failed to load default keywords: {}", err);
    }

    // Load custom keywords config if provided
    if args.len() >= 3 {
        let keyword_config = &args[2];
        if let Err(err) = keywords::load_keywords_from_file(keyword_config) {
            eprintln!("Warning: Failed to load keywords config: {}", err);
        } else {
            println!("Loaded custom keywords from {}", keyword_config);
        }
    }

    // Read the file
    let file_path = &args[1];
    println!("Running file: {}", file_path);

    let source = fs::read_to_string(file_path)
        .into_diagnostic()
        .map_err(|err| {
            miette::miette!(code = "tung::file_error", "Failed to read file: {}", err)
        })?;

    // Preprocess the source to replace aliases with original keywords
    let preprocessed_source = keywords::preprocess_source(&source);

    // Debug output to see the preprocessed source
    println!("Original source:\n{}", source);
    println!("Preprocessed source:\n{}", preprocessed_source);

    // Parse the file
    let pairs = TungParser::parse(Rule::file, &preprocessed_source).map_err(|err| {
        let message = format!("Parse error: {}", err);
        TungError::ParserError(message, None)
    })?;

    // Create an interpreter with the source for better error reporting
    // Use the original source, not preprocessed, for error reporting
    let mut interpreter = Interpreter::with_source(source.clone());

    // Interpret the parsed file
    if let Err(err) = interpreter.interpret(pairs) {
        // Let miette handle the error display
        return Err(err.into());
    }

    Ok(())
}
