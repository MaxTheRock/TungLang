#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i32),
    String(String),
    Undefined,
}
