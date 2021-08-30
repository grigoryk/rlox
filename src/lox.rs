use std::io;
use text_io::read;

use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    fn report(&self, line_number: usize, loc: String, message: &str) {
        println!("[line {}] Error {}: {}", line_number, loc, message);
    }

    pub fn error(&mut self, line_number: usize, message: &str) {
        self.report(line_number, String::from(""), message);
        self.had_error = true;
    }

    fn run(&mut self, source: &str) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens(self);
        for token in tokens {
            print!("{}", token);
        }
    }

    pub fn run_file(&mut self, path: std::path::PathBuf) -> io::Result<()> {
        println!("Running {:?}", path);
        let source = std::fs::read_to_string(&path)?;
        self.run(&source);
        println!("");
        if self.had_error {
            println!("Error during scanning, exit...");
            std::process::exit(1);
        }
        Ok(())
    }

    pub fn repl(&mut self) {
        println!("Welcome to Lox!");
        println!("--------------");
        loop {
            let line: String = read!("{}\n");
            self.run(&line);
            self.had_error = false;
        }
    }
}
