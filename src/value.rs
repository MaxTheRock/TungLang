use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
}

impl Value {
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
        }
    }
}

// Implement Display trait to allow printing values with println!("{}", value)
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}
