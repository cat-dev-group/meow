//! The reference implementation of Meow is a bytecode interpreter. The
//! interpreter in it's current state goes through the following primary
//! phases.
//!
//! lexing -> ...
//!
//! Each of these phases may contain more specific steps, documented within
//! their respective modules

pub mod errors;
pub mod lexer;

use anyhow::Result;
use lexer::Lexer;
use std::{fs, path::Path};

/// Create an instance of [`Lexer`](lexer::Lexer). This doesn't evaluate
/// anything itself, but exists for testing an
pub fn lex(source: &str) -> Lexer {
    Lexer::new(source)
}

pub fn parse(source: &str) {
    let mut lexer = lex(source);

    while let Some(token) = lexer.next() {
        println!("{}", token)
    }
}

pub fn run_from_file(path: &str) -> Result<()> {
    let filename = Path::new(path);
    let contents = fs::read_to_string(filename)?;

    run(&contents);

    Ok(())
}

pub fn run(source: &str) {
    parse(source)
}
