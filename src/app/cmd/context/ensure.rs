use std::cmp::max;
use crate::app::cmd::CmdContext;
use crate::error::PanicResult;
use crate::lexer;

impl CmdContext<'_> {
    pub fn token_len_at_least(&self, len: usize) -> PanicResult<()> {
        let len = max(1, len);
        
        if self.batch.tokens().len() < len {
            self.raise_raw(format!(
                "Command {} requires at least {} arguments",
                lexer::definition_of(self.cmd.name()), len,
            ))?
        }
        
        Ok(())
    }
}