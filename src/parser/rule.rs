#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rule {
    file,
    statement,
    assignment,
    function_call,
    expression,
    binary_expr,
    identifier,
    value,
    operator,
    EOI
}