use crate::error::RuntimeResult;
use crate::app::cmd::context::CmdContext;
use super::define_cmd;

define_cmd!(Exit, "exit");

impl Exit {
    fn execute(ctx: CmdContext) -> RuntimeResult<()> {
        if ctx.tokens_len() >= 2 {
            let code = ctx.parse_token(1)?;
            ctx.raise_failure_code(code)?
        }

        ctx.raise_success_code()?
    }
}