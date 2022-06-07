//pub mod ast_printer;
pub mod error;
pub mod expr;
pub mod interpreter;
pub mod lit;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;
pub mod token_type;
pub mod environment;

use std::{
    env::args,
    fs::File,
    io::{self, BufRead, Read, Write},
};

//use ast_printer::AstPrinter;
use error::*;
use interpreter::*;
use parser::Parser;
use scanner::*;

fn main() {
    let args = args().collect::<Vec<String>>();
    let lox = Lox::new(Interpreter::new());
    match args.len() {
        1 => {
            lox.run_prompt().unwrap();
        }
        2 => {
            lox.run_file(&args[1]).expect("Unable to open file");
        }
        _ => {
            println!("Usage: rlox [SCRIPT]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new(interpreter: Interpreter) -> Self {
        Self { interpreter }
    }

    pub fn run_file(&self, path: &str) -> io::Result<()> {
        let mut inp = String::new();
        File::open(path)?.read_to_string(&mut inp).unwrap();
        if self.run(inp).is_err() {
            // Ignore - error was already reported
            std::process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&self) -> io::Result<()> {
        let stdin = std::io::stdin();
        let handle = stdin.lock();

        print!("> ");
        std::io::stdout().flush().unwrap();
        for line in handle.lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                if self.run(line).is_err() {
                    // Ignore - error was already reported
                }
                print!("> ");
                std::io::stdout().flush().unwrap();
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn run(&self, source: String) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);

        let statements = parser.parse()?;

        if parser.success() {
            self.interpreter.interpret(&statements);
        }
        Ok(())
    }
}
