use std::path::Path;
use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::define_native_cmd;

define_native_cmd!(Cd, "cd");

impl Cd {
    fn execute(ctx: CmdContext) -> RuntimeResult<String> {
        ctx.ensure_args_at_least(2)?;
        
        let path_str = &ctx.token_expansion_at(1);
        let path = Path::new(path_str);
        
        if let Err(err) = ctx.env.shell_mut().set_cd(path) {
            ctx.raise_at_token(format!(
                "Failed to change working directory: {}", err
            ), 1)?
        }
        
        Ok(String::new())
    } 
}