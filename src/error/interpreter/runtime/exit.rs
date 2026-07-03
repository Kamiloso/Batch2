use thiserror::Error;

pub type ExitResult<T> = Result<T, ExitError>;

#[derive(Error, Debug)]
#[error("Exited with code: {code}")]
pub struct ExitError {
    pub code: u8,
}

impl ExitError {
    pub fn new(code: u8) -> Self {
        Self { code }
    }
    
    pub fn raise<T>(self) -> Result<T, Self> {
        Err(self)
    }
}