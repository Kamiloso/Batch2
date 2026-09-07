use std::marker::PhantomData;
use crate::types::{ShapeParse, ShapeStringify};
use crate::tools::chars::{BRACKET_OPEN, BRACKET_CLOSE};
use crate::lexer;

pub struct ArrayShape;

impl ShapeParse for ArrayShape {
    type Output = Vec<String>;
    
    fn parse(value: &str) -> Option<Vec<String>> {
        let inner_text = value
            .strip_prefix(BRACKET_OPEN)?
            .strip_suffix(BRACKET_CLOSE)?;

        lexer::split_token_line(inner_text)
    }
}

impl ShapeStringify for ArrayShape {
    type Input<'t> = &'t [String];
    
    fn stringify(value: &[String]) -> String {
        let joined_tokens = lexer::join_tokens_parsable(value);
        format!("{}{}{}", BRACKET_OPEN, joined_tokens, BRACKET_CLOSE)
    }
}