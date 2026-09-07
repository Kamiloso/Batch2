use std::vec::IntoIter;
use std::borrow::Cow;
use crate::structs::{Batch, BatchBuf, Token};
use crate::error::SyntaxResult;
use crate::tools::chars;
use crate::tools::escaping;
use crate::tools::expansion;
use crate::types::{BracketShape, ArrayShape};
use tokenizer::BatchTokenizer;

mod builder;
mod state;
mod tokenizer;
mod context;

pub fn compile(source_code: &str) -> SyntaxResult<Vec<BatchBuf>> {
    BatchTokenizer::new(source_code, true).split_into_batches()
}

pub fn compile_ok(source_code: &str) -> Option<Vec<BatchBuf>> {
    BatchTokenizer::new(source_code, false).split_into_batches().ok()
}

pub fn parse_definition(definition: &str) -> Option<String> {
    let batches = compile_ok(definition)?;
    let tokens = (batches.len() == 1).then(|| batches[0].tokens())?;

    (tokens.len() == 1).then(|| String::from(tokens[0].as_ref()))
}

pub fn split_token_line(token_line: &str) -> Option<Vec<String>> {
    let batches = compile_ok(token_line)?;

    Some(batches
        .iter()
        .flat_map(|batch| batch.tokens().iter())
        .map(|t| t.to_value())
        .collect()
    )
}

pub fn get_definition_display(text: impl AsRef<str>) -> String {
    get_definition(text, false)
}

pub fn get_definition_parsable(text: impl AsRef<str>) -> String {
    get_definition(text, true)
}

pub fn join_tokens_display(tokens: &[impl AsRef<str>]) -> String {
    join_tokens(tokens, false)
}

pub fn join_tokens_parsable(tokens: &[impl AsRef<str>]) -> String {
    join_tokens(tokens, true)
}

fn get_definition(text: impl AsRef<str>, duplicate_expansion: bool) -> String {
    let text_ref = text.as_ref();

    let literal: Cow<str> = if duplicate_expansion {
        Cow::Owned(expansion::duplicate_expansion_chars(text_ref))
    } else {
        Cow::Borrowed(text_ref)
    };

    if parse_definition(&literal).is_some_and(|parsed| parsed == literal.as_ref()) {
        return literal.into_owned();
    }

    let escaped = escaping::escape_quoted(&literal);
    if parse_definition(&escaped).is_some_and(|parsed| parsed == literal.as_ref()) {
        return escaped;
    }

    panic!("all definition strategies failed")
}

fn join_tokens(tokens: &[impl AsRef<str>], duplicate_expansion: bool) -> String {
    tokens
        .as_ref()
        .iter()
        .map(|text| get_definition(text, duplicate_expansion))
        .collect::<Vec<_>>()
        .join(" ")
}