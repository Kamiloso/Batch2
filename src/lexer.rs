use crate::structs::Batch;
use crate::error::SyntaxResult;
use tokenizer::BatchTokenizer;
use escaping::escape;
use crate::utils::chars;

mod builder;
mod state;
mod tokenizer;
mod context;
mod escaping;

pub fn split_source_code(source_code: &str) -> SyntaxResult<Vec<Batch>> {
    BatchTokenizer::new(source_code).split_into_batches()
}

pub fn definition_of(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    
    if value.chars().all(chars::is_solid) {
        String::from(value)
    } else {
        format!("\"{}\"", escape(&value))
    }
}

pub fn code_snippet_of<T>(value: impl AsRef<[T]>) -> String
where
    T: AsRef<str>,
{
    value.as_ref().iter()
        .map(definition_of)
        .collect::<Vec<String>>()
        .join(" ")
}