use crate::types::{NumberShape, ShapeParse, ShapeStringify};
use crate::utils::regex;

pub struct BoolShape;

impl ShapeParse for BoolShape {
    type Output = bool;
    
    fn parse(value: &str) -> Option<bool> {
        match NumberShape::parse(value)? {
            0 => Some(false),
            1 => Some(true),
            _ => None
        }
    }
}

impl ShapeStringify for BoolShape {
    type Input<'t> = bool;
    
    fn stringify(value: bool) -> String {
        if value {
            NumberShape::stringify(1)
        } else {
            NumberShape::stringify(0)
        }
    }
}