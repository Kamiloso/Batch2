use crate::types::{ShapeParse, ShapeStringify};
use crate::utils::regex;

pub struct NumberShape;

impl ShapeParse for NumberShape {
    type Output = i64;
    
    fn parse(value: &str) -> Option<i64> {
        value.parse::<i64>().ok()
    }
}

impl ShapeStringify for NumberShape {
    type Input<'t> = i64;
    
    fn stringify(value: i64) -> String {
        value.to_string()
    }
}