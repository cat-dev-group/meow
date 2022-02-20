//! Errror reporting for Meow. This module contains utilities and
//! functions for creating and reporting different errors

use ansi_term::Colour::{Red, Yellow};
use std::fmt;

/// An enum which contains the possible error kinds
pub enum ErrorKind {
    /// Errors in the parser related to invalid syntax
    InvalidSyntax,
    /// Errors in which the parser didn't find an expected token
    ExpectedToken,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::InvalidSyntax => write!(f, "Invalid Syntax"),
            ErrorKind::ExpectedToken => write!(f, "Expected token"),
        }
    }
}

/// A label which corresponds to some source code
/// to be put in the error message
pub struct Label<'a> {
    // The start of the label in the source code
    pub start: usize,
    // The end of the label in the source code
    pub end: usize,
    // The text for the label
    pub text: &'a str,
}

impl<'a> Label<'a> {
    /// Creates a new `Label, and is typically used to add labels
    /// to snippets of code in an error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use meow::errors::{ErrorKind, Label, Responder};
    ///
    /// let source = "
    /// fun main() {
    ///     println(\"Hello, World)
    /// }
    /// ";
    ///
    /// let responder = Responder::new(source.to_string());
    /// let label = Label::new(14, 38, "Unclosed String");
    ///
    /// responder.emit_error(
    ///     ErrorKind::InvalidSyntax,
    ///     2,
    ///     18,
    ///     "main.mw",
    ///     vec![label],
    ///     "Expected closing double quote `\"`",
    /// );
    /// ```
    pub fn new(start: usize, end: usize, text: &'a str) -> Self {
        Self { start, end, text }
    }
}

/// Used for creating and emitting error messages
pub struct Responder {
    source: String,
}

impl Responder {
    /// Creates a `Responder`, and is typically used to
    /// create and emit error messages.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use meow::errors::{ErrorKind, Label, Responder};
    ///
    /// let source = "
    /// fun main() {
    ///     println(\"Hello, World)
    /// }
    /// ";
    ///
    /// let responder = Responder::new(source.to_string());
    /// let label = Label::new(14, 38, "Unclosed String");
    ///
    /// responder.emit_error(
    ///     ErrorKind::InvalidSyntax,
    ///     2,
    ///     18,
    ///     "main.mw",
    ///     vec![label],
    ///     "Expected closing double quote `\"`",
    /// );
    /// ```
    pub fn new(source: String) -> Self {
        Self { source }
    }

    /// Emits an error message to the standard error (stderr). Usees the
    /// source string provided to the `Responder` it was invoked on.
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
        eprintln!("- {}", kind);
        eprintln!(
            "= at {}\n",
            Yellow.paint(format!("{}:{}:{}", filename, line, column))
        );

        for label in labels.iter() {
            eprintln!(
                "| {}\n|{}^^-- {}",
                &self.source[label.start..label.end],
                " ".repeat(label.start + 1),
                Red.paint(label.text)
            )
        }

        eprintln!("{}", Yellow.paint(format!("\n= note: {}", message)));
    }
}
