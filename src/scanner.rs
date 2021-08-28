use crate::{
    lox::Lox,
    types::{Token, TokenKind},
};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self, lox: &mut Lox) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(lox);
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::from(""),
            literal: None,
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn scan_token(&mut self, lox: &mut Lox) {
        match self.advance() {
            '(' => self.add_token(TokenKind::LeftParen, None),
            ')' => self.add_token(TokenKind::RightParen, None),
            '{' => self.add_token(TokenKind::LeftBrace, None),
            '}' => self.add_token(TokenKind::RightBrace, None),
            ',' => self.add_token(TokenKind::Comma, None),
            '.' => self.add_token(TokenKind::Dot, None),
            '-' => self.add_token(TokenKind::Minus, None),
            '+' => self.add_token(TokenKind::Plus, None),
            ';' => self.add_token(TokenKind::Semicolon, None),
            '*' => self.add_token(TokenKind::Star, None),
            _ => lox.error(self.line, String::from("Unexpected character")),
        }
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect(format!("source out of bounds at {}", self.current).as_str());
        self.current += 1;
        c
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            kind: kind,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        })
    }
}
