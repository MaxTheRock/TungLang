use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
        }
    }
}

// Implement Display trait to allow printing values with println!("{}", value)
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}
