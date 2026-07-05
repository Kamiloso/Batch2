use crate::structs::{Batch, Token};
use crate::app::Environment;
use crate::app::shell::Shell;
use crate::error::{ExitError, ExitResult, PanicError, PanicResult};

pub struct RuntimeContext<'a> {
    scope: &'a mut Environment,
    batch: Batch,
}

impl<'a> RuntimeContext<'a> {
    pub fn scope(&self) -> &Environment { self.scope }
    pub fn scope_mut(&mut self) -> &mut Environment { self.scope }
    pub fn shell(&self) -> &Shell { self.scope.shell() }
    pub fn shell_mut(&mut self) -> &mut Shell { self.scope.shell_mut() }
    pub fn batch(&self) -> &Batch { &self.batch }
    pub fn tokens(&self) -> &Vec<Token> { self.batch.tokens() }

    pub fn new(scope: &'a mut Environment, batch: Batch) -> Self {
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