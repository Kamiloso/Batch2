use std::any;
use std::str::FromStr;
use crate::app::cmd::context::CmdContext;
use crate::error::PanicResult;
use crate::lexer;
use crate::structs::Token;

impl CmdContext<'_> {
    pub fn parse_token<F: FromStr>(&self, token: &Token) -> PanicResult<F> {
        self.parse_token_slice::<F>(token, token.value())
    }

    pub fn parse_token_slice<F: FromStr>(&self, token: &Token, s: &str) -> PanicResult<F> {
        match s.parse::<F>() {
            Ok(f) => Ok(f),
            Err(_) => self.raise_at_token(
                format!(
                    "Value {} cannot be converted into {}",
                    lexer::definition_of(s), any::type_name::<F>()
                ),
                token,
            )?
        }
    }
}