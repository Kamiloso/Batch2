use crate::error::{ExitError, ExitResult, PanicError, PanicResult};
use crate::lexer;
use super::CmdContext;

impl CmdContext<'_, '_> {
    pub fn raise_raw<T>(&self, msg: impl Into<String>) -> PanicResult<T> {
        let tokens = self.batch.tokens();
        
        PanicError::new(msg, self.batch.lin_index())
            .with_batch_str(lexer::join_tokens_display(tokens))
            .raise()?
    }

    pub fn raise_at_token<T>(&self, msg: impl Into<String>, idx: usize) -> PanicResult<T> {
        let token = self.token_at(idx);
        let tokens = self.batch.tokens();
        
        PanicError::new(msg, token.lin_index())
            .with_tok_index(token.tok_index())
            .with_token_str(lexer::get_definition_display(token))
            .with_batch_str(lexer::join_tokens_display(tokens))
            .raise()?
    }

    pub fn raise_failure_code<T>(&self, code: u8) -> ExitResult<T> {
        ExitError::new(code).raise()?
    }

    pub fn raise_success_code<T>(&self) -> ExitResult<T> {
        ExitError::new(0).raise()?
    }
}