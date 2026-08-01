use crate::types::DuckParser;

pub struct StringType;

impl DuckParser for StringType {
    type Ref<'a> = &'a str;

    fn parse(value: &str) -> Option<&str> {
        Some(value)
    }
    
    fn stringify(value: &str) -> String {
        String::from(value)
    }
}