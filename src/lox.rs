use text_io::read;

use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }
}

impl Lox {
    fn report(&self, line_number: usize, loc: String, message: String) {
        println!("[line {}] Error {}: {}", line_number, loc, message);
    }

    pub fn error(&mut self, line_number: usize, message: String) {
        self.report(line_number, String::from(""), message);
        self.had_error = true;
    }

    fn run(&mut self, line: String) {
        let mut scanner = Scanner::new(line);
        scanner.scan_tokens(self);
        for token in scanner.tokens {
            println!("{:?}", token);
        }
    }

    pub fn run_file(&mut self, path: std::path::PathBuf) {
        println!("Running {:?}", path);
        let content = std::fs::read_to_string(&path).expect("could not read file");
        for line in content.lines() {
            self.run(line.to_string());
            if self.had_error {
                std::process::exit(1);
            }
        }
    }

    pub fn repl(&mut self) {
        println!("Welcome to Lox");
        println!("--------------");
        loop {
            let line: String = read!("{}\n");
            self.run(line);
            self.had_error = false;
        }
    }
}
