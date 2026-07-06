use crate::error::RuntimeResult;
use crate::app::cmd::context::CmdContext;
use crate::define_native_cmd;

define_native_cmd!(Exit, "exit");

impl Exit {
    fn execute(ctx: CmdContext) -> RuntimeResult<String> {
        let tokens = ctx.batch.tokens();

        if tokens.len() <= 1 {
            ctx.raise_success_code()?
        }

        let code = ctx.parse_token(&tokens[1])?;
        ctx.raise_failure_code(code)?
    }
}