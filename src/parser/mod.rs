//! This file contains Meow's parser. The parser uses the Lexer to generate tokens and parse them
//! into [`AST`](crate::parser::ast.rs) (Abstract Syntax Tree) nodes on demand. The abstract syntax tree is a tree-like
//! representation of Meow's syntax.

pub mod ast;
pub mod expression;
pub mod precedence;

use crate::errors::{ErrorKind, Label, Responder};
use crate::lexer::{
    token::{Token, TokenKind},
    Lexer,
};
use crate::parser::ast::Stmt;

/// The `Span` struct represents the span
/// (start and end) of an [`AST`](crate::parser::ast.rs) (Abstract Syntax Tree)
/// node in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    // Start of the span
    pub start: usize,
    // End of the span
    pub end: usize,
}

/// The `Position` struct represents the given position
/// of an [`AST`](crate::parser::ast.rs) (Abstract Syntax Tree) node
/// in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    span: Span,
    pub line: u32,
    pub column: u32,
}

impl Position {
    /// Creates a new `Position` instance, which is typically
    /// used as the `position` argument on an [`AST`](crate::parser::ast.rs) (Abstract Syntax Tree) node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use meow::parser::{Position, Span};
    ///
    /// let span = Span { start: 1, end: 5 };
    /// let position = Position::new(span, 1, 1);
    /// ```
    pub fn new(span: Span, line: u32, column: u32) -> Self {
        Self { span, line, column }
    }

    /// Returns the `start` position on the inner Span struct
    pub fn start(&self) -> usize {
        self.span.start
    }

    /// Returns the `end` position on the inner Span struct
    pub fn end(&self) -> usize {
        self.span.end
    }
}

/// The `Parser` struct uses the given Lexer struct and
/// parses them, generating [`AST`](crate::parser::ast.rs) (Abstract Syntax Tree)
/// nodes. This generated AST is then put through semantic analysis,
/// and eventually is compiled down to Meow bytecode.
pub struct Parser<'a> {
    /// The Abstract Syntax Tree (AST) which the parser fills up as it goes on.
    pub ast: Vec<Stmt>,
    // The next token in token stream
    pub next: Option<Token>,
    // The current token in the token stream
    pub current: Option<Token>,
    // The [`Responder`](crate::errors::Responder)
    responder: Responder,
    // The [`Lexer`](crate::lexer::Lexer) that the parser uses to scan tokens on demand
    lexer: Lexer<'a>,
    // The file that the parser recieved the source code from
    filename: &'a str,
}

impl<'a> Parser<'a> {
    /// Creates a `Parser`, which takes a source string as an argument,
    /// and is used to generate an AST (Abstract Syntax Tree) from the
    /// source string.
    pub fn new(source: &'a str, filename: &'a str) -> Self {
        // Create the responder
        let responder = Responder::new(source.to_string());
        // Create the lexer
        let lexer = Lexer::new(source);

        Self {
            ast: vec![],
            next: None,
            current: None,
            responder,
            lexer,
            filename,
        }
    }

    /// A wrapper around the lexer's `next_token` method, which
    /// advances the token stream and returns the next [`Token`](crate::lexer::token::Token)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use meow::parser::Parser;
    /// use meow::lexer::token::TokenKind;
    ///
    /// let mut parser = Parser::new("25", "main.mw");
    /// parser.advance();
    ///
    /// assert_eq!(parser.next.unwrap().kind, TokenKind::Int("25".to_string()));
    /// ```
    pub fn advance(&mut self) {
        self.current = self.next.take();

        // Loop until the next token isn't an error, and then break
        loop {
            self.next = Some(self.lexer.next_token());

            match &self.next.as_ref().unwrap().kind {
                TokenKind::Error(message) => self.parser_error(
                    ErrorKind::InvalidSyntax,
                    self.next.as_ref().unwrap(),
                    vec![],
                    &message,
                ),
                _ => break,
            }
        }
    }

    /// Advances to the next token in the token stream, and validates whether the token
    /// has the expected kind, and creates and emits an error if it doesn't.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use meow::parser::Parser;
    /// use meow::lexer::token::TokenKind;
    /// use meow::errors::Label;
    ///
    /// let mut parser = Parser::new("let x 10", "main.mw");
    ///
    /// parser.advance();
    /// parser.advance();
    ///
    /// let label = Label::new(6, 7, "Unexpected token");
    /// parser.consume(TokenKind::Equal, "Expected to find token `=`", vec![label]);
    /// ```
    pub fn consume(&mut self, expected: TokenKind, message: &str, labels: Vec<Label>) {
        // Safe to unwrap, since consume isn't called by the parser until an advance has been called
        let token = self.next.as_ref().unwrap();

        if token.kind != expected {
            self.responder.emit_error(
                ErrorKind::InvalidSyntax,
                token.line,
                token.column,
                self.filename,
                labels,
                message,
            )
        }

        self.advance();
    }

    // Wrapper around the parser's `Responder` for constructing and emitting an error
    fn parser_error(&self, kind: ErrorKind, token: &Token, labels: Vec<Label>, message: &str) {
        self.responder.emit_error(
            kind,
            token.line,
            token.column,
            self.filename,
            labels,
            message,
        )
    }
}
