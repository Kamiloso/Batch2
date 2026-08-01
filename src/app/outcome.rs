use std::fmt;
use std::process::ExitCode;
use crate::error::{InterpreterError, InterpreterResult, RuntimeError};

pub enum Outcome {
    Continue,
    Failure { display: Box<dyn fmt::Display> },
    Exit { code: ExitCode },
}

impl From<InterpreterResult<()>> for Outcome {
    fn from(result: InterpreterResult<()>) -> Outcome {
        if result.is_ok() {
            return Outcome::Continue;
        }
        
        match result.unwrap_err() {
            InterpreterError::Syntax(syntax) => {
                Outcome::Failure { display: Box::new(syntax) }
            },
            InterpreterError::Runtime(runtime) => {
                match runtime {
                    RuntimeError::Panic(panic) => {
                        Outcome::Failure { display: Box::new(panic) }
                    },
                    RuntimeError::Exit(exit) => {
                        Outcome::Exit { code: ExitCode::from(exit.code) }
                    },
                }
            }
        }
    }
}