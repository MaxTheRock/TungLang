use pest::iterators::Pairs;

use crate::diagnostics::span_to_source_span;
use crate::interpreter::{Interpreter, InterpreterError};
use crate::parser::Rule;

impl Interpreter {
    pub(super) fn handle_function_call(
        &mut self,
        mut pairs: Pairs<Rule>,
    ) -> Result<(), InterpreterError> {
        let id_pair = pairs.next();
        let function_name = match id_pair {
            Some(ref id_pair) if id_pair.as_rule() == Rule::identifier => id_pair.as_str(),
            _ => {
                let span = id_pair.as_ref().map(|p| span_to_source_span(p.as_span()));
                return Err(InterpreterError::InvalidExpression(
                    "Expected function name in function call".to_string(),
                    span,
                ));
            }
        };

        // Special handling for print function
        if function_name == "print" {
            // Collect arguments, handling each result
            let mut args = Vec::new();
            for p in pairs {
                if p.as_rule() == Rule::expression {
                    args.push(self.evaluate_expression(p.into_inner())?);
                }
            }

            // Print each argument
            for arg in args {
                println!("{}", arg);
            }
            Ok(())
        } else {
            let span = id_pair.as_ref().map(|p| span_to_source_span(p.as_span()));
            Err(InterpreterError::InvalidExpression(
                format!(
                    "Unknown function: '{}'. Available functions: print",
                    function_name
                ),
                span,
            ))
        }
    }
}
