use std::fmt;

use crate::scanner::ScanIndex;

#[derive(Debug)]
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
                formatted = format!("\"{}\"", self.literal.unwrap());
                &formatted
            },
            TokenKind::Number => {
                formatted = format!("{}", self.numeric_literal.unwrap());
                &formatted
            },
            TokenKind::And => "&&",
            TokenKind::Class => self.literal.unwrap(),
            TokenKind::Else => "else",
            TokenKind::False => "false",
            TokenKind::Fun => "fun",
            TokenKind::For => "for",
            TokenKind::If => "if",
            TokenKind::Nil => "nil",
            TokenKind::Or => "||",
            TokenKind::Print => "print",
            TokenKind::Return => "return",
            TokenKind::Super => "super",
            TokenKind::This => "this",
            TokenKind::True => "true",
            TokenKind::Var => "var",
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
                if scan_index.start == scan_index.start + length {
                    None
                } else {
                    match kind {
                        TokenKind::String => {
                            Some(&source[scan_index.start + 1..scan_index.start + length])
                        }
                        TokenKind::Number => {
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
