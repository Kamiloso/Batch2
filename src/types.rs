mod number;
mod list;
mod string;
mod code;
mod range;

use crate::utils::regex;
pub use number::*;
pub use list::*;
pub use string::*;
pub use range::*;
pub use code::*;

pub trait DuckParser {
    type Ref<'a>;

    fn parse(value: &str) -> Option<Self::Ref<'_>>;
    fn stringify(value: Self::Ref<'_>) -> String;
}