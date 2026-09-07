use crate::app::cmd::CmdContext;
use crate::app::cmd::context::EvalPair;
use crate::error::RuntimeResult;
use super::define_cmd;

define_cmd!(Echo, "echo");

impl Echo {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<()> {
        let joined = ctx.eval_join(1)?.result + "\n";
        
        ctx.stream_mut().print(&joined);

        Ok(())
    }
}