use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::app::cmd::context::EvalPair;
use super::define_cmd;

define_cmd!(If, "if");

impl If {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(2)?;
        
        if let EvalPair {
            result: true,
            next_idx
        } = ctx.eval_logic(1)? {
            return Ok(ctx.run_from(next_idx)?);
        }

        Ok(())
    }
}