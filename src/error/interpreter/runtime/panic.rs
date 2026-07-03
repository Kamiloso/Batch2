use std::fmt;
use thiserror::Error;

pub type PanicResult<T> = Result<T, PanicError>;

#[derive(Error, Debug)]
pub struct PanicError {
    pub message: String,
    pub lin_index: usize,
    pub tok_index: Option<usize>,
    pub token_str: Option<String>,
    pub batch_str: Option<String>,
}

impl PanicError {
    pub fn new(message: impl Into<String>, lin_index: usize) -> Self {
        Self {
            message: message.into(),
            lin_index,
            tok_index: None,
            token_str: None,
            batch_str: None,
        }
    }
    
    pub fn with_tok_index(mut self, tok_index: usize) -> Self {
        self.tok_index = Some(tok_index);
        self
    }

    pub fn with_token_str(mut self, token_str: impl Into<String>) -> Self {
        self.token_str = Some(token_str.into());
        self
    }

    pub fn with_batch_str(mut self, batch_str: impl Into<String>) -> Self {
        self.batch_str = Some(batch_str.into());
        self
    }

    pub fn raise<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl fmt::Display for PanicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Panic error at line {}", self.lin_index)?;

        if let Some(tok_index) = self.tok_index {
            write!(f, ", token {}", tok_index)?;
        }
        
        write!(f, "\n{}", self.message)?;

        if let Some(token_str) = &self.token_str {
            write!(f, "\nToken: {token_str}")?;
        }

        if let Some(batch_str) = &self.batch_str {
            write!(f, "\nBatch: {batch_str}")?;
        }

        Ok(())
    }
}