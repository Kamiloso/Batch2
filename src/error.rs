#![allow(unused_imports)]

mod interpreter;

use thiserror::Error;
pub use interpreter::{InterpreterError, *};

pub type AnyResult<T> = Result<T, AnyError>;

#[derive(Error, Debug)]
pub enum AnyError {
    #[error(transparent)]
    Interpreter(#[from] InterpreterError),
}