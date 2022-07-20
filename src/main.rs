mod lexer;
mod repl;
use std::io;
use crate::repl::repl::start;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let input = io::stdin();
    let output = io::stdout();
    let result = start(input.lock(), output.lock());
    result
}
