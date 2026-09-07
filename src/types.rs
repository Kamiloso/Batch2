mod number;
mod array;
mod bracket;
mod range;
mod exit;
mod bool;

use crate::utils::regex;
use crate::tools::chars;
use crate::tools::chars::{BRACKET_OPEN, BRACKET_CLOSE, QUOTE};
use crate::tools::escaping;
pub use number::*;
pub use array::*;
pub use bracket::*;
pub use range::*;
pub use exit::*;
pub use bool::*;

pub trait ShapeParse {
    type Output;
    
    fn parse(text: &str) -> Option<Self::Output>;
}

pub trait ShapeStringify {
    type Input<'t>;
    
    fn stringify(value: Self::Input<'_>) -> String;
}