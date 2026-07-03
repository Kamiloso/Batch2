use std::fmt;
use thiserror::Error;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Error, Debug)]
pub struct SyntaxError {
    pub message: String,
    pub lin_index: usize,
    pub col_index: usize,
    pub slice_str: Option<String>,
}

impl SyntaxError {
    pub fn new(message: impl Into<String>, lin_index: usize, col_index: usize) -> Self {
        Self {
            message: message.into(),
            lin_index,
            col_index,
            slice_str: None,
        }
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
        
        if let Some(slice_str) = &self.slice_str {
            if slice_str.chars().count() == 1 { // TODO: fix character / slice display
                write!(f, "\nCharacter: {slice_str}")?;
            } else {
                write!(f, "\nSlice: {slice_str}")?;
            }
        }
        
        Ok(())
    }
}