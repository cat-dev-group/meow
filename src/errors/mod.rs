//! Errror reporting for Meow. This module contains utilities and
//! functions for creating and reporting different errors
use ansi_term::Colour::{Red, Yellow};
use std::fmt;

// An enum which contains different error types
pub enum ErrorKind {
    // Errors in the parser related to invalid syntax
    InvalidSyntax,
}

// Display impl for ErrorKind which prints out the corresponding message for the current error kind
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::InvalidSyntax => write!(f, "Invalid Syntax"),
        }
    }
}

// A label for some text in the error message
pub struct Label<'a> {
    // The start of the label in the source code
    pub start: usize,
    // The end of the label in the source code
    pub end: usize,
    // The text for the label
    pub text: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(start: usize, end: usize, text: &'a str) -> Self {
        Self { start, end, text }
    }
}

// A struct used for reporting error messages
pub struct Responder {
    source: String,
}

impl Responder {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn emit_error(
        &self,
        kind: ErrorKind,
        line: u32,
        column: u32,
        filename: &str,
        labels: Vec<Label>,
        message: &str,
    ) {
        eprint!("{} ", Red.paint("Error"));
        eprint!("- {}\n", kind);
        eprint!( 
            "= at {}\n",
            Yellow.paint(format!("{}:{}:{}", filename, line, column))
        );

        for label in labels.iter() {
            eprint!(
                "| {} -- {}\n",
                &self.source[label.start..label.end],
                Red.paint(label.text)
            )
        }

        eprintln!("{}", Yellow.paint(format!("\n= note: {}", message)));
    }
}
