use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

use crate::scanner::ScanIndex;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenKind> = {
        let mut m = HashMap::new();
        m.insert("and", TokenKind::And);
        m.insert("class", TokenKind::Class);
        m.insert("else", TokenKind::Else);
        m.insert("false", TokenKind::False);
        m.insert("fun", TokenKind::Fun);
        m.insert("for", TokenKind::For);
        m.insert("if", TokenKind::If);
        m.insert("nil", TokenKind::Nil);
        m.insert("or", TokenKind::Or);
        m.insert("print", TokenKind::Print);
        m.insert("return", TokenKind::Return);
        m.insert("super", TokenKind::Super);
        m.insert("this", TokenKind::This);
        m.insert("true", TokenKind::True);
        m.insert("var", TokenKind::Var);
        m.insert("while", TokenKind::While);
        m
    };
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    // Single-character tokens,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character token,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals,
    Identifier,
    String,
    Number,

    // Keywords,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // ... and that's it.
    Eof,
}

impl TokenKind {
    fn is_keyword(&self) -> bool {
        match self {
            TokenKind::And
            | TokenKind::Class
            | TokenKind::Else
            | TokenKind::False
            | TokenKind::Fun
            | TokenKind::For
            | TokenKind::If
            | TokenKind::Nil
            | TokenKind::Or
            | TokenKind::Print
            | TokenKind::Return
            | TokenKind::Super
            | TokenKind::This
            | TokenKind::True
            | TokenKind::Var
            | TokenKind::While => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: Option<&'a str>,
    pub literal: Option<&'a str>,
    pub numeric_literal: Option<f64>,
    pub line: usize,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted: String;
        let s = match self.kind {
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Minus => "-",
            TokenKind::Plus => "+",
            TokenKind::Semicolon => ";",
            TokenKind::Slash => "/",
            TokenKind::Star => "*",
            TokenKind::Bang => "!",
            TokenKind::BangEqual => "!=",
            TokenKind::Equal => "=",
            TokenKind::EqualEqual => "==",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",
            TokenKind::Identifier => self.literal.unwrap(),
            TokenKind::String => {
                formatted = format!("\"{}\"", match self.literal {
                    Some(l) => l,
                    None => ""
                });
                &formatted
            }
            TokenKind::Number => {
                formatted = format!("{}", self.numeric_literal.unwrap());
                &formatted
            }
            TokenKind::And => "&&",
            TokenKind::Class => self.literal.unwrap(),
            TokenKind::Else => "else",
            TokenKind::False => "false",
            TokenKind::Fun => "fun",
            TokenKind::For => "for",
            TokenKind::If => "if",
            TokenKind::Nil => "nil",
            TokenKind::Or => "||",
            TokenKind::Print => "print ",
            TokenKind::Return => "return",
            TokenKind::Super => "super",
            TokenKind::This => "this",
            TokenKind::True => "true",
            TokenKind::Var => "var ",
            TokenKind::While => "while",
            TokenKind::Eof => "\0",
        };
        write!(f, "{}", s)
    }
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenKind,
        source: &'a str,
        scan_index: &ScanIndex,
        literal_length: Option<usize>,
    ) -> Token<'a> {
        let text = &source[scan_index.start..scan_index.current];
        let literal = match literal_length {
            None => None,
            Some(length) => {
                if length == 0 {
                    None
                } else {
                    match kind {
                        TokenKind::String => {
                            Some(&source[scan_index.start + 1..scan_index.start + length + 1])
                        }
                        TokenKind::Number | TokenKind::Identifier => {
                            Some(&source[scan_index.start..scan_index.start + length])
                        }
                        kind if kind.is_keyword() => {
                            Some(&source[scan_index.start..scan_index.start + length])
                        }
                        _ => None,
                    }
                }
            }
        };
        let numeric_literal = match kind {
            TokenKind::Number => Some(
                literal
                    .expect("number can't be empty")
                    .parse()
                    .expect("invalid number literal"),
            ),
            _ => None,
        };
        Token {
            kind,
            lexeme: Some(text),
            literal,
            numeric_literal,
            line: scan_index.line,
        }
    }
}
