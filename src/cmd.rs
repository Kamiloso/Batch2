mod echo;
mod cls;
mod exit;

use crate::structs::Batch;
use crate::errors::Bt2Result;
pub use echo::Echo;
pub use cls::Cls;
pub use exit::Exit;

pub trait Cmd {
    const NAME: &'static str;
    const MIN: usize;
    fn execute(batch: &Batch) -> Bt2Result<String>;

    fn pre_assert(batch: &Batch) -> Bt2Result<()> {
        let tokens = batch.as_ref();
        
        assert!(tokens.len() >= 1);
        assert_eq!(tokens[0].as_ref(), Self::NAME);
        
        if tokens.len() < Self::MIN {
            batch.syntax_error(
                &format!(
                    "Command {} requires at least {} arguments but only {} was provided.",
                    Self::NAME, Self::MIN, tokens.len()
                )
            ).raise()?;
        }
        
        Ok(())
    }
}