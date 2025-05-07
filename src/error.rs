use miette::{Diagnostic, SourceSpan};
use pest::error::Error as PestError;
use thiserror::Error;

use crate::parser::Rule;

#[derive(Error, Diagnostic, Debug)]
pub enum TungError {
    #[error("Parse error: {message}")]
    #[diagnostic(code(tung::parse_error))]
    Parse {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
        message: String,
    },

    #[error("Runtime error: {message}")]
    #[diagnostic(code(tung::runtime_error))]
    Runtime {
        #[source_code]
        src: String,
        #[label("in this expression")]
        span: Option<SourceSpan>,
        message: String,
    },

    #[error("Type error: {message}")]
    #[diagnostic(code(tung::type_error))]
    Type {
        #[source_code]
        src: String,
        #[label("expected a different type")]
        span: Option<SourceSpan>,
        message: String,
    },
}

impl From<PestError<Rule>> for TungError {
    fn from(err: PestError<Rule>) -> Self {
        let message = err.variant.to_string();
        let location = err.line_col;
        // We need to get the source from elsewhere, as PestError doesn't expose it directly
        // Use a placeholder for now, to be filled in from the context
        let src = String::new(); // This will be set by the caller

        // Calculate the span for the error
        let pos: (usize, usize) = match location {
            pest::error::LineColLocation::Pos((line, col)) => {
                // Return a tuple for position and length 1
                (calculate_position(&src, line, col), 1)
            }
            pest::error::LineColLocation::Span((line_start, col_start), (line_end, col_end)) => {
                let start = calculate_position(&src, line_start, col_start);
                let end = calculate_position(&src, line_end, col_end);
                (start, end - start)
            }
        };

        TungError::Parse {
            src,
            span: pos.into(),
            message,
        }
    }
}

// Helper function to calculate byte position from line and column
fn calculate_position(src: &str, line: usize, column: usize) -> usize {
    let mut current_line = 1;
    let mut current_pos = 0;

    for (i, c) in src.char_indices() {
        if current_line == line {
            if current_pos + 1 == column {
                return i;
            }
            current_pos += 1;
        }

        if c == '\n' {
            current_line += 1;
            current_pos = 0;
        }
    }

    // Default to the end of the string if the position can't be found
    src.len()
}

pub type Result<T> = miette::Result<T>;

// Helper function to convert pest error with source
pub fn pest_error_with_source(err: PestError<Rule>, source: String) -> TungError {
    let mut error = TungError::from(err);
    if let TungError::Parse { ref mut src, .. } = error {
        *src = source;
    }
    error
}
