use std::cmp::max;
use crate::error::PanicResult;
use crate::lexer;
use super::CmdContext;

impl CmdContext<'_, '_> {
    pub fn ensure_args_at_least(&self, len: usize) -> PanicResult<()> {
        let len = max(1, len);
        
        if self.tokens_len() < len {
            let name = self.cmd.name();
            let name_definition = lexer::get_definition_display(name);
            
            self.raise_raw(format!(
                "Command {} requires at least {} arguments",
                name_definition, len,
            ))?
        }
        
        Ok(())
    }
    
    pub fn ensure_arg_is(&self, idx: usize, expected: &str) -> PanicResult<()> {
        let value = self.token_value_at(idx);
        
        if value != expected {
            let expected_definition = lexer::get_definition_display(expected);
            let value_definition = lexer::get_definition_display(value);
            
            self.raise_at_token(format!(
                "Argument {} must be {}, but was {}",
                idx, expected_definition, value_definition
            ), idx)?
        }
        
        Ok(())
    }
}