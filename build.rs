mod generate_ast;
use std::io;

use generate_ast::*;
pub fn main() -> io::Result<()> {
    generate_ast("src")
}