use crate::structs::{Batch, Token};
use crate::app::Scope;
use crate::error::{ExitError, ExitResult, PanicError, PanicResult};

pub struct BatchContext<'a> {
    scope: &'a mut Scope,
    batch: Batch,
}

impl<'a> BatchContext<'a> {
    pub fn scope_mut(&mut self) -> &mut Scope { self.scope }
    pub fn batch(&self) -> &Batch { &self.batch }
    pub fn tokens(&self) -> &Vec<Token> { self.batch.tokens() }

    pub fn new(scope: &'a mut Scope, batch: Batch) -> Self {
        Self { scope, batch }
    }

    pub fn raise_raw<T>(&self, msg: impl Into<String>) -> PanicResult<T> {
        PanicError::new(msg, self.batch.lin_index())
            .with_batch_str(self.batch.to_code_snippet())
            .raise()?
    }

    pub fn raise_at_token<T>(&self, msg: impl Into<String>, token: &Token) -> PanicResult<T> {
        PanicError::new(msg, self.batch.lin_index())
            .with_tok_index(token.tok_index())
            .with_token_str(token.to_definition())
            .with_batch_str(self.batch.to_code_snippet())
            .raise()?
    }

    pub fn raise_failure_code<T>(&self, code: u8) -> ExitResult<T> {
        ExitError::new(code).raise()?
    }

    pub fn raise_success_code<T>(&self) -> ExitResult<T> {
        ExitError::new(0).raise()?
    }
}