use crate::types::DuckParser;

type SplitReturn<'a> = std::str::SplitWhitespace<'a>;

pub struct ListType;

impl DuckParser for ListType {
    type Ref<'a> = SplitReturn<'a>;

    fn parse(value: &str) -> Option<SplitReturn<'_>> {
        Some(value.split_whitespace()) // TODO: fix array temporary split by whitespace
    }
    
    fn stringify(value: SplitReturn<'_>) -> String {
        value.collect::<Vec<_>>().join(" ")
    }
}