use crate::utils;
use crate::structs::{Batch, Token};
use crate::errors::Bt2Result;
use super::Cmd;

pub struct Echo {}

impl Cmd for Echo {
    const NAME: &str = "echo";
    const MIN: usize = 1;

    fn execute(batch: &Batch) -> Bt2Result<String> {
        Self::pre_assert(batch)?;
        
        let tokens = batch.as_ref();
        
        let mut result = String::new();
        utils::fmt_join(
            &mut result,
            tokens[1..].iter().map(Token::as_ref),
            " "
        ).unwrap();
        
        Ok(result + "\n")
    }
}