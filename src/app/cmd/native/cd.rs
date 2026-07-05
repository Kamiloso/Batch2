use std::path::Path;
use crate::error::RuntimeResult;
use crate::app::context::RuntimeContext;
use crate::app::cmd::{Cmd, cmd_pre_assert};

pub struct Cd;

impl Cmd for Cd {
    fn name(&self) -> &'static str { "cd" }
    fn min(&self) -> usize { 2 }
    
    fn execute(&self, mut context: RuntimeContext) -> RuntimeResult<String> {
        cmd_pre_assert::<Self>(self, &context)?;
        
        // TODO: make it better, I'm tired for now
        
        let tokens = context.tokens().clone();
        
        let path_token = &tokens[1];
        let path_string = path_token.value();
        let path = Path::new(&path_string);
        
        match context.shell_mut().set_cwd(path) {
            Ok(()) => Ok(String::new()),
            Err(err) => context.raise_at_token("CD error has occurred: TODO better", path_token)?
        }
    }
}