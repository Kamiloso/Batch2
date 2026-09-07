use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::types::NumberShape;
use super::define_cmd;

define_cmd!(Loop, "loop");

impl Loop {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(2)?;
        
        let repeats = ctx.parse_token::<NumberShape>(1)?;
        
        for _ in 0..repeats {
            ctx.run_from(2)?;
        }

        Ok(())
    }
}