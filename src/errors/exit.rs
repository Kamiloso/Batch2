use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use super::{Bt2Error, Bt2Result};

#[derive(Debug)]
pub struct ExitError(u8);

impl ExitError {
    pub fn code(&self) -> u8 { self.0 }
    
    pub fn new(code: u8) -> Self {
        Self(code)
    }
    
    pub fn new_success() -> Self {
        Self(0)
    }
    
    pub fn raise<T>(self) -> Bt2Result<T> {
        Err(Bt2Error::Exit(self))
    }
}

impl Display for ExitError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "--- EXIT ERROR ---\nExit code: {}", self.0)
    }
}

impl Error for ExitError {}