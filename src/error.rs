use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Span {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl Span {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    pub fn from_token(token: &Token) -> Self {
        Self {
            start_line: token.start_line,
            start_col: token.start_col,
            end_line: token.end_line,
            end_col: token.end_col,
        }
    }

    pub fn from_two_token(start: &Token, end: &Token) -> Self {
        Self {
            start_line: start.start_line,
            start_col: start.start_col,
            end_line: end.end_line,
            end_col: end.end_col,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompileError {
    pub span: Span,
    pub msg: String,
}
