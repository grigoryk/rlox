use std::fmt;

#[derive(Debug)]
enum TokenKind {
    // Single-character tokens,
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character token,
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals,
    Identifier, String, Number,

    // Keywords,
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While,

    // ... and that's it.
    Eof
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: String,
    line: u32
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.kind, self.lexeme, self.literal)
    }
}
