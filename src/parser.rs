use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/tung.pest"]
pub struct TungParser;
