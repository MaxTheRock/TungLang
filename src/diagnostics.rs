use miette::{Diagnostic, SourceSpan};
use pest::Span;
use thiserror::Error;

/// Convert a pest Span to a miette SourceSpan
pub fn span_to_source_span(span: Span) -> SourceSpan {
    let start = span.start();
    let end = span.end();
    (start, end - start).into()
}

#[derive(Debug, Error, Diagnostic)]
pub enum TungError {
    #[error("Variable not found: {0}")]
    #[diagnostic(code(tung::variable_not_found))]
    VariableNotFound(
        String,
        #[label("Variable referenced here")] Option<SourceSpan>,
    ),

    #[error("Type mismatch: {0}")]
    #[diagnostic(code(tung::type_mismatch))]
    TypeMismatch(
        String,
        #[label("Type error occurred here")] Option<SourceSpan>,
    ),

    #[error("Division by zero")]
    #[diagnostic(code(tung::division_by_zero))]
    DivisionByZero(#[label("Division by zero attempted here")] Option<SourceSpan>),

    #[error("Invalid operator: {0}")]
    #[diagnostic(code(tung::invalid_operator))]
    InvalidOperator(
        String,
        #[label("Invalid operator used here")] Option<SourceSpan>,
    ),

    #[error("Invalid expression: {0}")]
    #[diagnostic(code(tung::invalid_expression))]
    InvalidExpression(
        String,
        #[label("Error in this expression")] Option<SourceSpan>,
    ),

    #[error("Invalid statement: {0}")]
    #[diagnostic(code(tung::invalid_statement))]
    InvalidStatement(
        String,
        #[label("Error in this statement")] Option<SourceSpan>,
    ),

    #[error("Parser error: {0}")]
    #[diagnostic(code(tung::parser_error))]
    ParserError(String, #[label("Parsing failed here")] Option<SourceSpan>),
}
