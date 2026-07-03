#![allow(unused_imports)]

mod panic;
mod exit;

use thiserror::Error;
pub use panic::{PanicError, *};
pub use exit::{ExitError, *};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    Panic(#[from] PanicError),

    #[error(transparent)]
    Exit(#[from] ExitError),
}