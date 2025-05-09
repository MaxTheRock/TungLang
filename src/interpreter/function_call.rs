use pest::iterators::Pairs;
use std::io::{self, Write};

use crate::diagnostics::TungError;
use crate::interpreter::expression::evaluate_expression;
use crate::interpreter::Interpreter;
use crate::keywords::resolve_function_name;
use crate::parser::Rule;
use crate::value::Value;

impl Interpreter {
    pub(super) fn handle_function_call(&mut self, mut pairs: Pairs<Rule>) -> Result<(), TungError> {
        // Get the function name
        let function_name = match pairs.next() {
            Some(id_pair) if id_pair.as_rule() == Rule::identifier => {
                let name = id_pair.as_str().to_string();
                // Resolve the function name using any aliases
                resolve_function_name(&name)
            }
            _ => {
                return Err(TungError::InvalidExpression(
                    "Expected function name".to_string(),
                    None,
                ))
            }
        };

        // Get the arguments
        let mut args = Vec::new();
        match pairs.next() {
            Some(args_pair) => {
                // Process each argument
                for p in args_pair.into_inner() {
                    if p.as_rule() == Rule::expression {
                        args.push(evaluate_expression(p.into_inner(), &self.variables)?);
                    }
                }
            }
            None => {
                return Err(TungError::InvalidExpression(
                    "Expected arguments list".to_string(),
                    None,
                ))
            }
        }

        // Handle built-in functions
        match function_name.as_str() {
            "print" => {
                // Print function: print all arguments to stdout
                for arg in args {
                    print!("{} ", arg);
                }
                println!();
                io::stdout().flush().unwrap();
            }
            "input" => {
                // Input function: prompt for input
                if !args.is_empty() {
                    // Print the prompt if provided
                    print!("{}", args[0]);
                    io::stdout().flush().unwrap();
                }

                // Read input from stdin
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();

                // Store the result in a special variable
                self.variables
                    .insert("result".to_string(), Value::String(input));
            }
            _ => {
                return Err(TungError::InvalidStatement(
                    format!("Unknown function: {}", function_name),
                    None,
                ))
            }
        }

        Ok(())
    }
}
