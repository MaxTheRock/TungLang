mod ast;
mod eval;
use crate::interpreter::run_program;
use clap::Parser as ClapParser;
use pest::Parser;
use pest_derive::Parser;
use std::fs;

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
    if path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.eq_ignore_ascii_case("tung"))
        != Some(true)
    {
        return Err(miette::miette!("Error: Only .tung files are allowed."));
    }

    let program: String = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(e) => {
            return Err(miette::miette!("Error reading file {}: {}", args.file, e));
        }
    };

    let parsed = match TungParser::parse(crate::Rule::program, &program) {
        Ok(parsed) => parsed,
        Err(e) => {
            // Let the caller / shell know something went wrong.
            return Err(miette::miette!("Error parsing program: {}", e));
        }
    };

    run_program(parsed)?;

    Ok(())
}
