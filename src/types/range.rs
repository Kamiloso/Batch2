use crate::types::DuckParser;

pub struct RangeType;

impl DuckParser for RangeType {
    type Ref<'a> = (i64, i64);

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
    
    fn stringify(value: (i64, i64)) -> String {
        format!("{}..{}", value.0, value.1)
    }
}