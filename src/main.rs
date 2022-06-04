pub mod ast_printer;
pub mod error;
pub mod expr;
pub mod interpreter;
pub mod lit;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod token_type;

use std::{
    env::args,
    fs::File,
    io::{self, BufRead, Read, Write},
};

use ast_printer::AstPrinter;
use error::*;
use parser::Parser;
use scanner::*;
use interpreter::*;

fn main() {
    let args = args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            run_prompt().unwrap();
        }
        2 => {
            run_file(&args[1]).expect("Unable to open file");
        }
        _ => {
            println!("Usage: rlox [SCRIPT]");
            std::process::exit(64);
        }
    }
}

pub fn run_file(path: &str) -> io::Result<()> {
    let mut inp = String::new();
    File::open(path)?.read_to_string(&mut inp).unwrap();
    if run(inp).is_err() {
        // Ignore - error was already reported
        std::process::exit(65);
    }
    Ok(())
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    print!("> ");
    std::io::stdout().flush().unwrap();
    for line in handle.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            if run(line).is_err() {
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

pub fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();

    match expr {
        Some(e) => {
            let printer = AstPrinter {};
            println!("{}", printer.format(&e)?);
        }
        None => {}
    }
    Ok(())
}
