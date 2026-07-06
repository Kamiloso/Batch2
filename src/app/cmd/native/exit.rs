use crate::error::RuntimeResult;
use crate::app::cmd::context::CmdContext;
use crate::define_native_cmd;

define_native_cmd!(Exit, "exit");

impl Exit {
    fn execute(ctx: CmdContext) -> RuntimeResult<String> {
        if ctx.token_count() >= 2 {
            let code = ctx.parse_token(1)?;
            ctx.raise_failure_code(code)?
        }

        ctx.raise_success_code()?
    }
}