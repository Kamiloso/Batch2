use crate::utils::cli;
use crate::error::RuntimeResult;
use crate::app::context::RuntimeContext;
use crate::app::cmd::{Cmd, cmd_pre_assert};

pub struct Cls;

impl Cmd for Cls {
    fn name(&self) -> &'static str { "cls" }
    fn min(&self) -> usize { 1 }
    
    fn execute(&self, context: RuntimeContext) -> RuntimeResult<String> {
        cmd_pre_assert::<Self>(self, &context)?;
        
        cli::clear!();
        
        Ok(String::new())
    }
}