mod syntax;
mod exit;

pub use syntax::SyntaxError;
pub use exit::ExitError;

pub enum Bt2Error {
    Syntax(SyntaxError),
    Exit(ExitError),
}

pub type Bt2Result<T> = Result<T, Bt2Error>;