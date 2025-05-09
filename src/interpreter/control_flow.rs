use pest::iterators::Pairs;

use crate::interpreter::{Interpreter, InterpreterError};
use crate::keywords::resolve_control_keyword;
use crate::parser::Rule;
use crate::value::Value;

impl Interpreter {
    pub(super) fn handle_if_statement(
        &mut self,
        mut pairs: Pairs<Rule>,
    ) -> Result<(), InterpreterError> {
        // Check the control keyword used (if or an alias)
        let control_word = match pairs.next() {
            Some(word) => resolve_control_keyword(word.as_str()),
            None => "if".to_string(), // Default to 'if' if missing
        };

        // First expression should be the condition
        let condition = match pairs.next() {
            Some(expr) => self.evaluate_expression(expr.into_inner())?,
            None => {
                return Err(InterpreterError::InvalidExpression(
                    format!(
                        "Missing condition in {} statement. Use: {} (condition):",
                        control_word, control_word
                    ),
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
