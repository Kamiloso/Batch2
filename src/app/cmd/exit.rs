use crate::error::RuntimeResult;
use crate::app::context::BatchContext;
use super::{Cmd, cmd_pre_assert, parse_token};

pub struct Exit;

impl Cmd for Exit {
    fn name() -> &'static str { "exit" }
    fn min() -> usize { 1 }
    
    fn execute(context: BatchContext) -> RuntimeResult<String> {
        cmd_pre_assert::<Self>(&context)?;

        let tokens = context.tokens();
        
        if tokens.len() <= 1 {
            context.raise_success_code()?
        }
        
        let code = parse_token(&context, &tokens[1])?;
        context.raise_failure_code(code)?
    }
}