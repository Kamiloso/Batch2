use crate::error::RuntimeResult;
use crate::app::context::RuntimeContext;
use crate::app::cmd::{Cmd, cmd_pre_assert, parse_token};

pub struct Exit;

impl Cmd for Exit {
    fn name(&self) -> &'static str { "exit" }
    fn min(&self) -> usize { 1 }
    
    fn execute(&self, context: RuntimeContext) -> RuntimeResult<String> {
        cmd_pre_assert::<Self>(self, &context)?;

        let tokens = context.tokens();
        
        if tokens.len() <= 1 {
            context.raise_success_code()?
        }
        
        let code = parse_token(&context, &tokens[1])?;
        context.raise_failure_code(code)?
    }
}