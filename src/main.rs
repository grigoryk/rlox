use std::fmt;
use structopt::StructOpt;
use text_io::read;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn report(&self, line_number: u32, loc: String, message: String) {
        println!("[line {}] Error {}: {}", line_number, loc, message);
    }
    
    fn error(&mut self, line_number: u32, message: String) {
        self.report(line_number, String::from(""), message);
        self.had_error = true;
    }

    fn run(&self, line: String) {
        let scanner = Scanner{};
        let tokens = scanner.scan_tokens(line);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    fn run_file(&self, path: std::path::PathBuf) {
        println!("Running {:?}", path);
        let content = std::fs::read_to_string(&path).expect("could not read file");
        for line in content.lines() {
            self.run(line.to_string());
            if self.had_error {
                std::process::exit(1);
            }
        }
    }

    fn repl(&mut self) {
        println!("Welcome to Lox");
        println!("--------------");
        loop {
            let line: String = read!("{}\n");
            self.run(line);
            self.had_error = false;
        }
    }
}

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

pub struct Scanner {}

impl Scanner {
    pub fn scan_tokens(&self, line: String) -> Vec<Token> {
        vec!()
    }
}

fn main() {
    let args = Cli::from_args();
    let mut lox = Lox { had_error: false };
    match args.path {
        Some(path) => lox.run_file(path),
        None => lox.repl()
    }
}
