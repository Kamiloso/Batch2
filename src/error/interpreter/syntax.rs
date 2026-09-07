use std::fmt;
use thiserror::Error;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Error, Debug)]
pub struct SyntaxError {
    pub message: String,
    pub lin_index: usize,
    pub col_index: usize,
    pub char_str: Option<String>,
    pub slice_str: Option<String>,
}

impl SyntaxError {
    pub fn new(message: impl Into<String>, lin_index: usize, col_index: usize) -> Self {
        Self {
            message: message.into(),
            lin_index,
            col_index,
            char_str: None,
            slice_str: None,
        }
    }
    
    pub fn any() -> Self {
        Self::new("", 0, 0)
    }
    
    pub fn with_char_str(mut self, char_str: impl Into<String>) -> Self {
        self.char_str = Some(char_str.into());
        self
    }

    pub fn with_slice_str(mut self, slice_str: impl Into<String>) -> Self {
        self.slice_str = Some(slice_str.into());
        self
    }

    pub fn raise<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Syntax error at line {}, column {}\n{}",
            self.lin_index, self.col_index, self.message
        )?;
        
        if let Some(char_str) = &self.char_str {
            write!(f, "\nCharacter: {char_str}")?;
        }
        
        if let Some(slice_str) = &self.slice_str {
            write!(f, "\nSlice: {slice_str}")?;
        }
        
        Ok(())
    }
}