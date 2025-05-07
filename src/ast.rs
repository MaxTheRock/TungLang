// AST nodes to represent our language constructs
#[derive(Debug)]
pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

#[derive(Debug)]
pub enum Statement {
    Assignment {
        name: String,
        value: Expr,
    },
    Print(Expr),
    Expression(Expr),
    If {
        condition: Expr,
        then_block: Vec<Statement>,
        else_block: Vec<Statement>,
    },
}
