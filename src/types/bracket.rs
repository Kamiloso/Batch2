use std::marker::PhantomData;
use crate::tools::chars;
use crate::types::{ShapeParse, ShapeStringify, NumberShape};
use crate::tools::chars::{BRACKET_OPEN, BRACKET_CLOSE};

pub struct BracketShape;

impl ShapeParse for BracketShape {
    type Output = String;
    
    fn parse(value: &str) -> Option<String> {
        Some(value
            .strip_prefix(BRACKET_OPEN)?
            .strip_suffix(BRACKET_CLOSE)?
            .trim()
            .into()
        )
    }
}

impl ShapeStringify for BracketShape {
    type Input<'t> = &'t str;
    
    fn stringify(value: &str) -> String {
        format!("{}\n{}\n{}", BRACKET_OPEN, value, BRACKET_CLOSE)
    }
}