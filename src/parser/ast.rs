//! The Abstract Syntax Tree for the Meow programming language. This file shows the corresponding AST for the grammar
//! in the grammar.txt file

use crate::parser::Position;

// The statement AST node
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    ExpressionStmt(Expr),
}

/// The expression AST node
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal {
        value: Lit,
        position: Position,
    },
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        position: Position,
    },
    Unary {
        op: UnaryOp,
        rhs: Box<Expr>,
        position: Position,
    },
    Ident(String),
    Grouping(Box<Expr>),
    Call {
        name: String,
        arguments: Vec<Expr>,
        position: Position,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        code: Box<Stmt>,
        _else: Option<Box<Stmt>>,
        position: Position,
    },
    Block {
        code: Vec<Stmt>,
        position: Position,
    },
    For {
        expr: Box<Expr>,
        code: Vec<Stmt>,
        position: Position,
    },
    While {
        expr: Box<Expr>,
        code: Vec<Stmt>,
        position: Position,
    },
    Match {
        expr: Box<Expr>,
        patterns: Vec<Case>,
    },
}

// The literal types
#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Integer(isize),
    Float(f64),
    Char(char),
    String(String),
    True,
    False,
}

// Available operators for binary operations
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

// Available operators for unary operations
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus,
    Bang,
}

// Representation of a case in a pattern matching expression
#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pattern: Box<Expr>,
    code: Vec<Stmt>,
}
