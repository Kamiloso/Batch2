use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use super::define_cmd;

define_cmd!(Set, "set");

impl Set {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(4)?;
        ctx.ensure_arg_is(2, "=")?;
        
        let var_name = &ctx.token_expansion_at(1);
        let var_value = &ctx.token_expansion_at(3);
        
        ctx.env_mut().set(var_name, var_value);
        
        Ok(())
    }
}