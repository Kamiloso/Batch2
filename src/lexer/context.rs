use crate::error::{SyntaxError, SyntaxResult};
use crate::lexer;
use crate::lexer::chars;

#[derive(Clone)]
pub struct SyntaxContext {
    last_char: Option<char>,
    lin_index: usize,
    col_index: usize,
}

impl SyntaxContext {
    pub fn new() -> Self {
        Self {
            last_char: None,
            lin_index: 1,
            col_index: 0,
        }
    }
    
    pub fn inject_char(&mut self, c: char) {
        self.col_index += 1;
        
        if let Some(last_char) = self.last_char {
            if chars::is_newline(last_char) {
                self.lin_index += 1;
                self.col_index = 1;
            }
        }

        self.last_char = Some(c);
    }
    
    pub fn raise_raw<T>(&self, message: impl Into<String>) -> SyntaxResult<T> {
        self.assert_injected();

        SyntaxError::new(message, self.lin_index, self.col_index)
            .raise()?
    }
    
    pub fn raise_at_char<T>(&self, message: impl Into<String>, c: char) -> SyntaxResult<T> {
        self.assert_injected();
        
        let seq = String::from(c);
        let seq_definition = &lexer::token_to_definition(&seq);

        SyntaxError::new(message, self.lin_index, self.col_index)
            .with_char_str(seq_definition)
            .raise()?
    }
    
    pub fn raise_at_seq<T>(&self, message: impl Into<String>, seq: &str) -> SyntaxResult<T> {
        self.assert_injected();
        
        let seq_definition = &lexer::token_to_definition(&seq);
        
        SyntaxError::new(message, self.lin_index, self.col_index)
            .with_slice_str(seq_definition)
            .raise()?
    }
    
    fn assert_injected(&self) {
        assert!(self.last_char.is_some(), "no character injected");
    }
}