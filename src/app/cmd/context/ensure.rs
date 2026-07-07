use std::cmp::max;
use crate::error::PanicResult;
use crate::lexer;
use super::CmdContext;

impl CmdContext<'_> {
    pub fn ensure_args_at_least(&self, len: usize) -> PanicResult<()> {
        let len = max(1, len);
        
        if self.tokens_len() < len {
            self.raise_raw(format!(
                "Command {} requires at least {} arguments",
                lexer::definition_of(self.cmd.name()), len,
            ))?
        }
        
        Ok(())
    }
    
    pub fn ensure_arg_is(&self, idx: usize, expected: &str) -> PanicResult<()> {
        let token = self.token_at(idx);
        let value = self.token_value_at(idx);
        
        if token.value() != expected {
            self.raise_at_token(format!(
                "Argument {} must be {}, but was {}",
                idx, lexer::definition_of(expected), lexer::definition_of(value)
            ), idx)?
        }
        
        Ok(())
    }
}