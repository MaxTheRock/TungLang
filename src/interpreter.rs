use crate::ast::{Expr, Operator, Statement};
use crate::error::{Result, TungError};
use std::collections::HashMap;

pub struct Interpreter {
    pub variables: HashMap<String, Value>,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Interpreter {
    pub fn new(source: String) -> Self {
        Self {
            variables: HashMap::new(),
            source,
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Identifier(name) => {
                if let Some(value) = self.variables.get(name) {
                    Ok(value.clone())
                } else {
                    Err(TungError::Runtime {
                        src: self.source.clone(),
                        span: None,
                        message: format!("Variable '{}' not defined", name),
                    }
                    .into())
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match (left_val, op, right_val) {
                    (Value::Number(l), Operator::Add, Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::Number(l), Operator::Subtract, Value::Number(r)) => {
                        Ok(Value::Number(l - r))
                    }
                    (Value::Number(l), Operator::Multiply, Value::Number(r)) => {
                        Ok(Value::Number(l * r))
                    }
                    (Value::Number(l), Operator::Divide, Value::Number(r)) => {
                        if r == 0.0 {
                            Err(TungError::Runtime {
                                src: self.source.clone(),
                                span: None,
                                message: "Division by zero".to_string(),
                            }
                            .into())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    (Value::Number(l), Operator::Equal, Value::Number(r)) => {
                        Ok(Value::Boolean(l == r))
                    }
                    (Value::String(l), Operator::Add, Value::String(r)) => {
                        Ok(Value::String(format!("{}{}", l, r)))
                    }
                    (Value::String(l), Operator::Equal, Value::String(r)) => {
                        Ok(Value::Boolean(l == r))
                    }
                    (Value::String(_), op, Value::String(_)) => Err(TungError::Type {
                        src: self.source.clone(),
                        span: None,
                        message: format!("Operator {:?} not supported for strings", op),
                    }
                    .into()),
                    _ => Err(TungError::Type {
                        src: self.source.clone(),
                        span: None,
                        message: "Type mismatch in binary operation".to_string(),
                    }
                    .into()),
                }
            }
        }
    }

    pub fn execute(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Assignment { name, value } => {
                let evaluated = self.evaluate(value)?;
                self.variables.insert(name.clone(), evaluated);
                Ok(())
            }
            Statement::Print(expr) => {
                let value = self.evaluate(expr)?;
                match value {
                    Value::Number(n) => println!("{}", n),
                    Value::String(s) => println!("{}", s),
                    Value::Boolean(b) => println!("{}", b),
                }
                Ok(())
            }
            Statement::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Statement::If {
                condition,
                then_block,
                else_block,
            } => {
                let condition_value = self.evaluate(condition)?;

                match condition_value {
                    Value::Boolean(true) => {
                        for stmt in then_block {
                            self.execute(stmt)?;
                        }
                    }
                    Value::Boolean(false) => {
                        for stmt in else_block {
                            self.execute(stmt)?;
                        }
                    }
                    _ => {
                        return Err(TungError::Type {
                            src: self.source.clone(),
                            span: None,
                            message: "Condition must evaluate to a boolean".to_string(),
                        }
                        .into());
                    }
                }

                Ok(())
            }
        }
    }
}
