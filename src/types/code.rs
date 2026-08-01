use crate::tools::chars;
use crate::types::{DuckParser, NumberType};
use crate::tools::chars::{BRACKET_OPEN, BRACKET_CLOSE};

pub struct CodeType;

impl DuckParser for CodeType {
    type Ref<'a> = &'a str;

    fn parse(value: &str) -> Option<&str> {
        Some(value
            .strip_prefix(BRACKET_OPEN)?
            .strip_suffix(BRACKET_CLOSE)?
            .trim()
        )
    }
    
    fn stringify(value: &str) -> String {
        format!("{}\n{}\n{}", BRACKET_OPEN, value, BRACKET_CLOSE)
    }
}