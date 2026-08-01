mod panic;
mod exit;

use thiserror::Error;
pub use panic::*;
pub use exit::*;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    Panic(#[from] PanicError),

    #[error(transparent)]
    Exit(#[from] ExitError),
}