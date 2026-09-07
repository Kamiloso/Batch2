use crate::types::{ShapeParse, ShapeStringify};
use crate::utils::regex;

pub struct ExitShape;

impl ShapeParse for ExitShape {
    type Output = u8;
    
    fn parse(value: &str) -> Option<u8> {
        value.parse::<u8>().ok()
    }
}

impl ShapeStringify for ExitShape {
    type Input<'t> = u8;
    
    fn stringify(value: u8) -> String {
        value.to_string()
    }
}