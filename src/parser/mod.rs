pub mod ast;

// Represents the span (start and end) of a node
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

// Represents the position of a node in the source code
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub span: Span,
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new(span: Span, line: u32, column: u32) -> Self {
        Self { span, line, column }
    }

    pub fn start(&self) -> usize {
        self.span.start
    }

    pub fn end(&self) -> usize {
        self.span.end
    }
}
