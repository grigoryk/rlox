use crate::{
    lox::Lox,
    types::{Token, TokenKind},
};

pub struct Scanner<'a> {
    pub source: &'a str,
}

#[derive(Debug)]
pub struct ScanIndex {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub source_length: usize,
}

impl ScanIndex {
    fn at_end(&self, offset: usize) -> bool {
        self.current + offset >= self.source_length
    }
}

enum ScanResult<'a> {
    SingleCharLexeme(Token<'a>),
    DoubleCharLexeme(Token<'a>),
    MultiCharLexeme(usize, Token<'a>),
    CommentLexeme(usize),
    StringLexeme(usize, usize, Token<'a>),
    Whitespace,
    Newline,
    Error(&'a str),
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner { source }
    }

    pub fn scan_tokens(&'a self, lox: &mut Lox) -> Vec<Token<'a>> {
        let mut tokens = vec![];
        let mut scan_index = ScanIndex {
            start: 0,
            current: 0,
            line: 1,
            source_length: self.source.chars().count(),
        };
        while !scan_index.at_end(0) {
            scan_index.start = scan_index.current;
            match self.scan_token(&scan_index) {
                ScanResult::SingleCharLexeme(token) => {
                    scan_index.current += 1;
                    tokens.push(token);
                }
                ScanResult::DoubleCharLexeme(token) => {
                    scan_index.current += 2;
                    tokens.push(token);
                }
                ScanResult::MultiCharLexeme(length, token) => {
                    scan_index.current += length;
                    tokens.push(token);
                }
                ScanResult::Whitespace => {
                    scan_index.current += 1;
                }
                ScanResult::Newline => {
                    scan_index.current += 1;
                    scan_index.line += 1;
                }
                ScanResult::CommentLexeme(length) => {
                    scan_index.current += length;
                }
                ScanResult::StringLexeme(length, extra_lines, token) => {
                    scan_index.current += length + 2; // length of string + ""
                    scan_index.line += extra_lines;
                    tokens.push(token);
                }
                ScanResult::Error(msg) => {
                    scan_index.current += 1;
                    lox.error(scan_index.line, msg);
                }
            };
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: None,
            literal: None,
            line: scan_index.line,
        });

        tokens
    }

    fn scan_token(&'a self, scan_index: &ScanIndex) -> ScanResult {
        match self.peek_offset(&scan_index, 0) {
            // whitespace
            Some(' ') | Some('\r') | Some('\t') => ScanResult::Whitespace,
            // newline
            Some('\n') => ScanResult::Newline,
            // single-character lexemes
            Some('(') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::LeftParen,
                self.source,
                scan_index,
                None,
            )),
            Some(')') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::RightParen,
                self.source,
                scan_index,
                None,
            )),
            Some('{') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::LeftBrace,
                self.source,
                scan_index,
                None,
            )),
            Some('}') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::RightBrace,
                self.source,
                scan_index,
                None,
            )),
            Some(',') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::Comma,
                self.source,
                scan_index,
                None,
            )),
            Some('.') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::Dot,
                self.source,
                scan_index,
                None,
            )),
            Some('-') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::Minus,
                self.source,
                scan_index,
                None,
            )),
            Some('+') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::Plus,
                self.source,
                scan_index,
                None,
            )),
            Some(';') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::Semicolon,
                self.source,
                scan_index,
                None,
            )),
            Some('*') => ScanResult::SingleCharLexeme(Token::new(
                TokenKind::Star,
                self.source,
                scan_index,
                None,
            )),
            Some('/') => {
                // if this is a single-line comment, denoted by //, figure out its length. Comment terminates either at newline or EOF.
                match self.peek_offset(scan_index, 1) {
                    Some('/') => {
                        let mut length = 2;
                        loop {
                            match self.peek_offset(scan_index, length) {
                                Some('\n') | None => {
                                    break ScanResult::CommentLexeme(length);
                                }
                                Some(_) => {
                                    length += 1;
                                }
                            }
                        }
                    }
                    _ => ScanResult::SingleCharLexeme(Token::new(
                        TokenKind::Slash,
                        self.source,
                        &scan_index,
                        None,
                    )),
                }
            }
            // single or two character lexemes
            Some('!') => match self.peek_offset(scan_index, 1) {
                Some('=') => ScanResult::DoubleCharLexeme(Token::new(
                    TokenKind::BangEqual,
                    self.source,
                    scan_index,
                    None,
                )),
                _ => ScanResult::SingleCharLexeme(Token::new(
                    TokenKind::Bang,
                    self.source,
                    scan_index,
                    None,
                )),
            },
            Some('=') => match self.peek_offset(scan_index, 1) {
                Some('=') => ScanResult::DoubleCharLexeme(Token::new(
                    TokenKind::EqualEqual,
                    self.source,
                    scan_index,
                    None,
                )),
                _ => ScanResult::SingleCharLexeme(Token::new(
                    TokenKind::Equal,
                    self.source,
                    scan_index,
                    None,
                )),
            },
            Some('<') => match self.peek_offset(scan_index, 1) {
                Some('=') => ScanResult::DoubleCharLexeme(Token::new(
                    TokenKind::LessEqual,
                    self.source,
                    scan_index,
                    None,
                )),
                _ => ScanResult::SingleCharLexeme(Token::new(
                    TokenKind::Less,
                    self.source,
                    scan_index,
                    None,
                )),
            },
            Some('>') => match self.peek_offset(scan_index, 1) {
                Some('=') => ScanResult::DoubleCharLexeme(Token::new(
                    TokenKind::GreaterEqual,
                    self.source,
                    scan_index,
                    None,
                )),
                _ => ScanResult::SingleCharLexeme(Token::new(
                    TokenKind::Greater,
                    self.source,
                    scan_index,
                    None,
                )),
            },

            // literals
            Some('"') => self.string(scan_index),

            _ => ScanResult::Error("Unexpected character"),
        }
    }

    fn string(&self, scan_index: &ScanIndex) -> ScanResult {
        // determine length of string
        let mut length = 0;
        // for multiline strings
        let mut extra_lines = 0;
        loop {
            match self.peek_offset(scan_index, length + 1) {
                Some('"') => {
                    break ScanResult::StringLexeme(
                        length,
                        extra_lines,
                        Token::new(TokenKind::String, self.source, scan_index, Some(length)),
                    )
                },
                Some('\n') => extra_lines += 1,
                Some(_) => {},
                None => break ScanResult::Error("Unterminated string"),
            }
            length += 1;
        }
    }

    fn peek_offset(&self, scan_index: &ScanIndex, offset: usize) -> Option<char> {
        return if scan_index.at_end(offset) {
            None
        } else {
            Some(self.source.chars().nth(scan_index.current + offset).expect(
                format!("source out of bounds at {}", scan_index.current + offset).as_str(),
            ))
        };
    }
}
