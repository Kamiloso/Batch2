use crate::structs::Batch;
use crate::errors::{Bt2Result, ExitError};
use super::Cmd;

pub struct Exit {}

impl Cmd for Exit {
    const NAME: &str = "exit";
    const MIN: usize = 1;
    
    fn execute(batch: &Batch) -> Bt2Result<String> {
        Self::pre_assert(batch)?;

        let tokens = batch.as_ref();
        
        if tokens.len() == 1 {
            return ExitError::new_success().raise()
        }
        
        let tk_exit_code = &tokens[1];

        match tk_exit_code.as_ref().parse::<u8>() {
            Ok(code) => {
                ExitError::new(code).raise()
            },
            Err(_) => {
                batch.syntax_error_at_token("Cannot parse u8.", tk_exit_code)
                    .raise()?
            }
        }
    }
}