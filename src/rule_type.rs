#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rule {
    // All rule variants that match the grammar.pest file
    File,
    Statement,
    Assignment,
    FunctionCall,
    Expression,
    BinaryExpr,
    Identifier,
    Value,
    Operator,
    EOI
}