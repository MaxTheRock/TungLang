use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "tung.pest"]
pub struct TungParser;
