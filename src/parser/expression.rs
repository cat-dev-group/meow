//! This file provides the functionality for parsing expressions, or some code which evaluates to some value.

use crate::errors::{ErrorKind, Label, Responder};
use crate::lexer::token::{Token, TokenKind};
use crate::parser::ast::{Expr, Lit};
use crate::parser::precedence::{get_precedence, Precedence};
use crate::parser::{Parser, Position, Span};

impl<'a> Parser<'a> {
    /// Entry point for parsing expressions
    pub fn parse_expression(&mut self, precedence: Precedence) {
        // Advance the token stream
        self.advance();

        // Safe to unwrap, since token stream has already been advanced, and the next field is not `None`
        let next = self.next.as_ref().unwrap();

        // Get the left hand side of a possible infix expression
        let lhs = match next.kind {
            TokenKind::Int(_) => self.parse_literal(),
            _ => todo!(),
        };

        let mut rhs: Expr;

        // Loop while current precedence is less than or equal to the precedence of the token being parsed
        while precedence.infix.unwrap_or(0)
            <= get_precedence(&self.next.as_ref().unwrap().kind)
                .infix
                .unwrap_or(0)
        {
            // Advance the token stream
            self.advance();
            self.parse_expression(get_precedence(&self.next.as_ref().unwrap().kind));
        }
    }

    // Parses a literal
    fn parse_literal(&mut self) -> Expr {
        let next = self.next.as_ref().unwrap();

        match &next.kind {
            TokenKind::Int(n) => {
                // Safe to unwrap, since value is confirmed to be a valid integer
                let value = n.parse::<isize>().unwrap();
                let position = Position {
                    span: Span {
                        start: next.start,
                        end: next.start + n.len(),
                    },
                    line: next.line,
                    column: next.column,
                };

                Expr::Literal {
                    value: Lit::Integer(value),
                    position,
                }
            }
            _ => unreachable!(),
        }
    }
}
