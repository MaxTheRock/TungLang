#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    Undefined,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub i64);

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanValue(pub bool);
