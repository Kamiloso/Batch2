use crate::app::cmd::CmdContext;
use crate::error::RuntimeResult;
use super::define_cmd;

define_cmd!(Echo, "echo");

impl Echo {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<()> {
        for i in 1..ctx.tokens_len() {
            let expansion = ctx.token_expansion_at(i);
            ctx.stream_mut().print(&expansion);
            
            if i != ctx.tokens_len() - 1 {
                ctx.stream_mut().print(" ");
            }
        }
        
        ctx.stream_mut().print("\n");

        Ok(())
    }
}