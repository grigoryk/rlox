use lazy_static::lazy_static;
use std::collections::HashMap;

use std::fmt;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut m = HashMap::new();
        m.insert("and", Keyword::And);
        m.insert("class", Keyword::Class);
        m.insert("else", Keyword::Else);
        m.insert("false", Keyword::False);
        m.insert("fun", Keyword::Fun);
        m.insert("for", Keyword::For);
        m.insert("if", Keyword::If);
        m.insert("nil", Keyword::Nil);
        m.insert("or", Keyword::Or);
        m.insert("print", Keyword::Print);
        m.insert("return", Keyword::Return);
        m.insert("super", Keyword::Super);
        m.insert("this", Keyword::This);
        m.insert("true", Keyword::True);
        m.insert("var", Keyword::Var);
        m.insert("while", Keyword::While);
        m
    };
}

impl From<&str> for Keyword {
    fn from(c: &str) -> Keyword {
        match c {
            "and" => Keyword::And,
            "class" => Keyword::Class,
            "else" => Keyword::Else,
            "false" => Keyword::False,
            "fun" => Keyword::Fun,
            "for" => Keyword::For,
            "if" => Keyword::If,
            "nil" => Keyword::Nil,
            "or" => Keyword::Or,
            "print" => Keyword::Print,
            "return" => Keyword::Return,
            "super" => Keyword::Super,
            "this" => Keyword::This,
            "true" => Keyword::True,
            "var" => Keyword::Var,
            "while" => Keyword::While,
            _ => panic!("unexpected keyword lexeme"),
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    Minus,
    Plus,
    Star,
    Equal,
    EqualEqual,
    Greater,
    Less,
    Slash,
    BangEqual,
    GreaterEqual,
    LessEqual,
    Bang,
}

#[derive(Debug)]
pub enum Grouping {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

#[derive(Debug)]
pub enum Misc {
    Comma,
    Dot,
    Semicolon,
}

#[derive(Clone, Copy, Debug)]
pub enum Keyword {
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
}

#[derive(Debug)]
pub enum Literal<'a> {
    Identifier { size: usize, literal: &'a str },
    String { size: usize, literal: &'a str },
    Number { literal: f64 },
}

#[derive(Debug)]
pub enum Token<'a> {
    Operator { line: usize, token: Operator },
    Grouping { line: usize, token: Grouping },
    Misc { line: usize, token: Misc },
    Literal { line: usize, token: Literal<'a> },
    Keyword { line: usize, token: Keyword },
    Eof { line: usize },
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Operator { token, .. } => write!(f, "{}", token),
            Token::Grouping { token, .. } => write!(f, "{}", token),
            Token::Misc { token, .. } => write!(f, "{}", token),
            Token::Literal { token, .. } => write!(f, "{}", token),
            Token::Keyword { token, .. } => write!(f, "{}", token),
            Token::Eof { .. } => Ok(()),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            Operator::Minus => "-",
            Operator::Plus => "+",
            Operator::Star => "*",
            Operator::Equal => "=",
            Operator::EqualEqual => "==",
            Operator::Greater => ">",
            Operator::Less => "<",
            Operator::Slash => "/",
            Operator::BangEqual => "!=",
            Operator::GreaterEqual => ">=",
            Operator::LessEqual => "<=",
            Operator::Bang => "!",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            Grouping::LeftParen => "(",
            Grouping::RightParen => ")",
            Grouping::LeftBrace => "{",
            Grouping::RightBrace => "}",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for Misc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            Misc::Comma => ",",
            Misc::Dot => ".",
            Misc::Semicolon => ";",
        };
        write!(f, "{}", op)
    }
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let number;
        let op = match self {
            Literal::Identifier { literal, .. } => literal,
            Literal::String { literal, .. } => literal,
            Literal::Number { literal } => {
                number = format!("{}", literal);
                number.as_str()
            }
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            Keyword::And => "and",
            Keyword::Class => "class",
            Keyword::Else => "else",
            Keyword::False => "false",
            Keyword::Fun => "fun",
            Keyword::For => "for",
            Keyword::If => "if",
            Keyword::Nil => "nil",
            Keyword::Or => "or",
            Keyword::Print => "print",
            Keyword::Return => "return",
            Keyword::Super => "super",
            Keyword::This => "this",
            Keyword::True => "true",
            Keyword::Var => "var",
            Keyword::While => "while",
        };
        write!(f, "{}", op)
    }
}
