use crate::utils::cli;
use crate::structs::Batch;
use crate::errors::Bt2Result;
use super::Cmd;

pub struct Cls {}

impl Cmd for Cls {
    const NAME: &str = "cls";
    const MIN: usize = 1;
    
    fn execute(batch: &Batch) -> Bt2Result<String> {
        Self::pre_assert(batch)?;
        
        cli::clear!();
        Ok(String::new())
    }
}