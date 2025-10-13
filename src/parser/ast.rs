//! src/parser/ast.rs

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Unary {
        op: Operator,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Grouped(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    VarDecl {
        name: String,
        value: Option<Expr>,
    },
    Return(Option<Expr>),
    Fn {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}