use pest::iterators::Pairs;

use crate::interpreter::{Interpreter, InterpreterError};
use crate::parser::Rule;
use crate::value::Value;

impl Interpreter {
    pub(super) fn handle_if_statement(
        &mut self,
        mut pairs: Pairs<Rule>,
    ) -> Result<(), InterpreterError> {
        // First pair should be the condition expression
        let condition = match pairs.next() {
            Some(expr) => {
                // Don't need to use the span here since the error will be reported
                // from inside evaluate_expression if needed
                self.evaluate_expression(expr.into_inner())?
            }
            None => {
                return Err(InterpreterError::InvalidExpression(
                    "Missing condition in if statement. Use: if (condition):".to_string(),
                    None,
                ));
            }
        };

        // Check if condition is true
        let condition_true = match condition {
            Value::Number(n) => n != 0,
            Value::String(s) => !s.is_empty(),
            Value::Boolean(b) => b,
        };

        // If condition is true, execute the statements inside the block
        if condition_true {
            for stmt in pairs {
                if stmt.as_rule() == Rule::statement {
                    self.handle_statement(stmt.into_inner())?;
                }
            }
        }

        Ok(())
    }
}
