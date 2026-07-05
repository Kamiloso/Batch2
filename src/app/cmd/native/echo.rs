use crate::error::RuntimeResult;
use crate::app::context::RuntimeContext;
use crate::app::cmd::{Cmd, cmd_pre_assert};

pub struct Echo;

impl Cmd for Echo {
    fn name(&self) -> &str { "echo" }
    fn min(&self) -> usize { 1 }

    fn execute(&self, context: RuntimeContext) -> RuntimeResult<String> {
        cmd_pre_assert::<Self>(self, &context)?;
        
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