use std::any;
use std::str::FromStr;
use crate::error::PanicResult;
use crate::lexer;
use super::CmdContext;

impl CmdContext<'_> {
    pub fn parse_token<F: FromStr>(&self, idx: usize) -> PanicResult<F> {
        let expansion = &self.token_expansion_at(idx);
        
        if let Ok(value) = expansion.parse::<F>() {
            return Ok(value);
        }

        self.raise_at_token(format!(
            "Value {} cannot be converted into {}",
            lexer::definition_of(expansion), any::type_name::<F>()
        ), idx)?
    }
}