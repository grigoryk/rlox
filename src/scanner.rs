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
            // whitespace
            ' ' | '\r' | '\t' => {},
            // newline
            '\n' => {
                println!("newline!");
                self.line += 1
            },
            // single-character lexemes
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
            '/' => {
                // if this is a comment, denoted by //, skip until end of the line
                if self.advance_if_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash, None)
                }
            },
            // single or two character lexemes
            '!' => {
                let kind = match self.advance_if_next('=') {
                    true => TokenKind::BangEqual,
                    false => TokenKind::Bang,
                };
                self.add_token(kind, None)
            },
            '=' => {
                let kind = match self.advance_if_next('=') {
                    true => TokenKind::EqualEqual,
                    false => TokenKind::Equal,
                };
                self.add_token(kind, None)
            },
            '<' => {
                let kind = match self.advance_if_next('=') {
                    true => TokenKind::LessEqual,
                    false => TokenKind::Less,
                };
                self.add_token(kind, None)
            },
            '>' => {
                let kind = match self.advance_if_next('=') {
                    true => TokenKind::GreaterEqual,
                    false => TokenKind::Greater,
                };
                self.add_token(kind, None)
            },

            _ => lox.error(self.line, format!("Unexpected character")),
        }
    }

    fn current_char(&self) -> char {
        self.source
            .chars()
            .nth(self.current)
            .expect(format!("source out of bounds at {}", self.current).as_str())
    }

    fn advance(&mut self) -> char {
        let c = self.current_char();
        self.current += 1;
        c
    }

    fn advance_if_next(&mut self, conditional: char) -> bool {
        return if self.is_at_end() {
            false
        } else if self.current_char() != conditional {
            false
        } else {
            self.current += 1;
            true
        };
    }

    fn peek(&self) -> char {
        return if self.is_at_end() {
            '\0'
        } else {
            self.current_char()
        }
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
