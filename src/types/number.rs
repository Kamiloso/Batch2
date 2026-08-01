use crate::types::DuckParser;

pub struct NumberType;

impl DuckParser for NumberType {
    type Ref<'a> = i64; // TODO: switch to big ints

    fn parse(value: &str) -> Option<i64> {
        value.parse::<i64>().ok()
    }
    
    fn stringify(value: i64) -> String {
        value.to_string()
    }
}