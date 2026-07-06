use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::define_native_cmd;

define_native_cmd!(Set, "set");

impl Set {
    fn execute(ctx: CmdContext) -> RuntimeResult<String> {
        ctx.ensure_args_at_least(4)?;
        ctx.ensure_arg_is(2, "=")?;
        
        ctx.env.set(
            ctx.token_expansion_at(1),
            ctx.token_expansion_at(3),
        );
        
        Ok(String::new())
    }
}