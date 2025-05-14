mod eval;
mod interpreter;
mod parser;
mod value;
mod stdlib;
use crate::interpreter::run_program;
use crate::parser::{Rule, TungParser};
use clap::Parser;
use pest::Parser as PestParserTrait;
use ::std::fs;
use ::std::path;
use ::std::ffi;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the TungLang source file
    #[arg(short, long)]
    pub file: String,
}

fn main() -> miette::Result<()> {
    let args: Args = Args::parse();

    let path: &path::Path = path::Path::new(&args.file);
    if path
        .extension()
        .and_then(|s: &ffi::OsStr| s.to_str())
        .map(|s: &str| s.eq_ignore_ascii_case("tung"))
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

    let parsed: pest::iterators::Pairs<Rule> = match TungParser::parse(Rule::program, &program) {
        Ok(mut pairs) => pairs.next().unwrap().into_inner(),
        Err(e) => {
            return Err(miette::miette!("Error parsing program: {}", e));
        }
    };

    run_program(parsed)?;

    Ok(())
}
