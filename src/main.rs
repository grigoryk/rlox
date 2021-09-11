mod lox;
mod parser;
mod scanner;
mod types;

use lox::Lox;
use parser::Expr;
use structopt::StructOpt;
use types::{Literal, Token};

use crate::parser::Parsed;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let mut lox = Lox::new();

    // playing with expression types...
    let exprs = Parsed {
        expressions: vec![
            Expr::Literal(Token::Literal {
                line: 0,
                token: Literal::Identifier {
                    size: 3,
                    literal: "gri",
                },
            }),
            Expr::Operator(Token::Operator {
                line: 0,
                token: types::Operator::Slash,
            }),
            Expr::Unary(
                Token::Operator {
                    line: 0,
                    token: types::Operator::Minus,
                },
                Box::new(Expr::Literal(Token::Literal {
                    line: 0,
                    token: Literal::Number { literal: 133.7 },
                })),
            ),
            Expr::Literal(Token::Literal {
                line: 0,
                token: Literal::Number { literal: 133.7 },
            }),
            Expr::Binary(
                Box::new(Expr::Literal(Token::Literal {
                    line: 0,
                    token: Literal::Number { literal: 1.7 },
                })),
                Token::Operator {
                    line: 0,
                    token: types::Operator::Star,
                },
                Box::new(Expr::Grouping(Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Token::Literal {
                        line: 0,
                        token: Literal::Number { literal: 1.7 },
                    })),
                    Token::Operator {
                        line: 0,
                        token: types::Operator::Star,
                    },
                    Box::new(Expr::Grouping(Box::new(Expr::Literal(Token::Literal {
                        line: 0,
                        token: Literal::Identifier {
                            size: 3,
                            literal: "gri",
                        },
                    })))),
                )))),
            ),
        ],
    };

    println!("{}", exprs);

    match args.path {
        Some(path) => lox.run_file(path).expect("error while running file"),
        None => lox.repl(),
    }
}
