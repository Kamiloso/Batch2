use crate::utils::cli;
use crate::error::RuntimeResult;
use crate::app::context::BatchContext;
use super::{Cmd, cmd_pre_assert};

pub struct Cls;

impl Cmd for Cls {
    fn name() -> &'static str { "cls" }
    fn min() -> usize { 1 }
    
    fn execute(context: BatchContext) -> RuntimeResult<String> {
        cmd_pre_assert::<Self>(&context)?;
        
        cli::clear!();
        
        Ok(String::new())
    }
}