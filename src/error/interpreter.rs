mod syntax;
mod runtime;

use thiserror::Error;
pub use syntax::*;
pub use runtime::*;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error(transparent)]
    Syntax(#[from] SyntaxError),
    
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
}