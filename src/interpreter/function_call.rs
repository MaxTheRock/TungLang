use pest::iterators::Pairs;

use crate::diagnostics::span_to_source_span;
use crate::interpreter::{Interpreter, InterpreterError};
use crate::keywords::resolve_function_name;
use crate::parser::Rule;

impl Interpreter {
    pub(super) fn handle_function_call(
        &mut self,
        mut pairs: Pairs<Rule>,
    ) -> Result<(), InterpreterError> {
        let id_pair = pairs.next();
        let function_alias = match id_pair {
            Some(ref id_pair) if id_pair.as_rule() == Rule::identifier => id_pair.as_str(),
            _ => {
                let span = id_pair.as_ref().map(|p| span_to_source_span(p.as_span()));
                return Err(InterpreterError::InvalidExpression(
                    "Expected function name in function call".to_string(),
                    span,
                ));
            }
        };

        // Resolve the standard function name from any alias
        let function_name = resolve_function_name(function_alias);

        // Standard function handlers - match on String by comparing with as_str()
        match function_name.as_str() {
            "print" => {
                // Collect arguments, handling each result
                let mut args = Vec::new();
                for p in pairs {
                    if p.as_rule() == Rule::expression {
                        args.push(self.evaluate_expression(p.into_inner())?);
                    }
                }

                // Print all arguments on the same line with spaces between them
                if !args.is_empty() {
                    let mut output = String::new();
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            output.push(' ');
                        }
                        output.push_str(&arg.to_string());
                    }
                    println!("{}", output);
                } else {
                    println!(); // Print empty line if no arguments
                }
                Ok(())
            }
            // Add more function implementations here
            _ => {
                let span = id_pair.as_ref().map(|p| span_to_source_span(p.as_span()));
                Err(InterpreterError::InvalidExpression(
                    format!(
                        "Unknown function: '{}'. Available functions: print",
                        function_alias
                    ),
                    span,
                ))
            }
        }
    }
}
