mod types;
mod lox;
mod scanner;

use std::fmt;
use structopt::StructOpt;
use types::Token;
use lox::Lox;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let mut lox = Lox::new();
    match args.path {
        Some(path) => lox.run_file(path),
        None => lox.repl()
    }
}
