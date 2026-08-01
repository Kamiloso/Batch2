use std::path::Path;
use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use super::define_cmd;

define_cmd!(Cd, "cd");

impl Cd {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(2)?;
        
        let path_str = &ctx.token_expansion_at(1);
        let path = Path::new(path_str);
        
        if let Err(err) = ctx.env_mut().shell_mut().set_cd(path) {
            ctx.raise_at_token(format!(
                "Failed to change working directory: {}", err
            ), 1)?
        }
        
        Ok(())
    }
}