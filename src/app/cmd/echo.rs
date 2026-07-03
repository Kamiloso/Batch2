use crate::error::RuntimeResult;
use crate::app::context::BatchContext;
use super::{Cmd, cmd_pre_assert};

pub struct Echo;

impl Cmd for Echo {
    fn name() -> &'static str { "echo" }
    fn min() -> usize { 1 }

    fn execute(context: BatchContext) -> RuntimeResult<String> {
        cmd_pre_assert::<Self>(&context)?;
        
        let tokens = context.tokens();
        let mut sb = String::new();
        
        for token in tokens[1..].iter() {
            sb.push_str(token.value());
            sb.push(' ');
        }
        
        if sb.ends_with(' ') {
            sb.pop();
        }
        
        Ok(sb + "\n")
    }
}