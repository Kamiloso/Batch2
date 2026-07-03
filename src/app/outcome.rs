use std::fmt;
use std::process::ExitCode;
use crate::error::{InterpreterError, InterpreterResult, RuntimeError};

pub enum Outcome {
    Continue { out: String },
    Failure { display: Box<dyn fmt::Display> },
    Exit { code: ExitCode },
}

impl From<InterpreterResult<String>> for Outcome {
    fn from(result: InterpreterResult<String>) -> Outcome {
        if let Ok(out) = result {
            return Outcome::Continue { out };
        }
        match result.unwrap_err() {
            InterpreterError::Syntax(syntax) => {
                Outcome::Failure { display: Box::new(syntax) }
            },
            InterpreterError::Runtime(RuntimeError::Panic(panic)) => {
                Outcome::Failure { display: Box::new(panic) }
            }
            InterpreterError::Runtime(RuntimeError::Exit(exit)) => {
                Outcome::Exit { code: ExitCode::from(exit.code) }
            },
        }
    }
}