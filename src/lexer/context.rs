use crate::error::{SyntaxError, SyntaxResult};
use crate::lexer;
use crate::tools::chars;

#[derive(Clone)]
pub struct SyntaxContext {
    robust_errors: bool,
    newline_before: bool,
    lin_index: usize,
    col_index: usize,
}

impl SyntaxContext {
    pub fn new(robust_errors: bool) -> Self {
        Self {
            robust_errors,
            newline_before: false,
            lin_index: 1,
            col_index: 0,
        }
    }

    pub fn observe_char(&mut self, c: char) {
        self.col_index += 1;

        if self.newline_before {
            self.lin_index += 1;
            self.col_index = 1;
        }

        self.newline_before = chars::is_newline(c);
    }

    pub fn raise_raw<T>(&self, message: impl Into<String>) -> SyntaxResult<T> {
        self.raise_if_robust(|| {
            SyntaxError::new(message, self.lin_index, self.col_index).raise()?
        })
    }

    pub fn raise_at_char<T>(&self, message: impl Into<String>, c: char) -> SyntaxResult<T> {
        self.raise_if_robust(|| {
            let seq = String::from(c);
            let seq_definition = lexer::get_definition_display(seq);

            SyntaxError::new(message, self.lin_index, self.col_index)
                .with_char_str(seq_definition)
                .raise()?
        })
    }

    pub fn raise_at_seq<T>(&self, message: impl Into<String>, seq: &str) -> SyntaxResult<T> {
        self.raise_if_robust(|| {
            let seq_definition = lexer::get_definition_display(seq);

            SyntaxError::new(message, self.lin_index, self.col_index)
                .with_slice_str(seq_definition)
                .raise()?
        })
    }

    fn raise_if_robust<T>(&self, get_error: impl FnOnce() -> SyntaxResult<T>) -> SyntaxResult<T> {
        if !self.robust_errors {
            SyntaxError::any().raise()?;
        }

        get_error()
    }
}