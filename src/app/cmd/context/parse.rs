use std::any;
use std::str::FromStr;
use crate::error::PanicResult;
use crate::lexer;
use super::CmdContext;

impl CmdContext<'_> {
    pub fn parse_token<F: FromStr>(&self, idx: usize) -> PanicResult<F> {
        let expansion = &self.token_expansion_at(idx);
        
        self.parse_token_slice::<F>(idx, expansion)
    }

    pub fn parse_token_slice<F: FromStr>(&self, idx: usize, slice: &str) -> PanicResult<F> {
        if let Ok(value) = slice.parse::<F>() {
            return Ok(value);
        }

        self.raise_at_token(format!(
            "Value {} cannot be converted into {}",
            lexer::definition_of(slice), any::type_name::<F>()
        ), idx)?
    }
}