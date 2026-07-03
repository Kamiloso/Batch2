use crate::structs::Batch;
use crate::error::SyntaxResult;
use tokenizer::BatchTokenizer;
use escaping::escape;

mod chars;
mod builder;
mod state;
mod tokenizer;
mod context;
mod escaping;

pub fn split_source_code(source_code: &str) -> SyntaxResult<Vec<Batch>> {
    BatchTokenizer::new(source_code).split_into_batches()
}

pub fn token_to_definition(token_str: &str) -> String {
    format!("\"{}\"", escape(token_str))
}