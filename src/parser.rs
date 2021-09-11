use std::fmt;

use crate::types::Token;

// simple grammar to start with:
// expr -> literal | binary | grouping | unary
// literal -> string | number | identifier
// binary -> expr operator expr
// grouping -> "(" expr ")"
// unary -> "-" | "!" expr
// operator -> "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/"

pub enum Expr<'a> {
    Literal(Token<'a>),
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Unary(Token<'a>, Box<Expr<'a>>),
    Operator(Token<'a>),
}

pub struct Parsed<'a> {
    pub expressions: Vec<Expr<'a>>,
}

impl<'a> fmt::Display for Parsed<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.expressions {
            write!(f, " ({}) ", e)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(token) => write!(f, "{}", token),
            Expr::Binary(left, token, right) => write!(f, "{} {} {}", left, token, right),
            Expr::Grouping(expr) => write!(f, "group({})", expr),
            Expr::Unary(operator, expr) => write!(f, "{} {}", operator, expr),
            Expr::Operator(operator) => write!(f, "{}", operator),
        }
    }
}
