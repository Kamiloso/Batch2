use crate::types::{ShapeParse, ShapeStringify};

pub struct RangeShape;

impl ShapeParse for RangeShape {
    type Output = (i64, i64);
    
    fn parse(value: &str) -> Option<(i64, i64)> {
        let split = value
            .split("..")
            .collect::<Vec<_>>();
        
        if split.len() != 2 {
            return None;
        }

        let beg = split[0].parse().ok()?;
        let end = split[1].parse().ok()?;
        
        Some((beg, end))
    }
}

impl ShapeStringify for RangeShape {
    type Input<'t> = (i64, i64);
    
    fn stringify(value: (i64, i64)) -> String {
        format!("{}..{}", value.0, value.1)
    }
}