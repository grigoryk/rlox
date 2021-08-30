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
    pub line: usize,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
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
                    Some(&source[scan_index.start + 1..scan_index.start + length])
                }
            }
        };
        Token {
            kind: kind,
            lexeme: Some(text),
            literal: literal,
            line: scan_index.line,
        }
    }
}
