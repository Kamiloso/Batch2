use std::path::Path;
use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::define_native_cmd;

define_native_cmd!(Cd, "cd");

impl Cd {
    fn execute(ctx: CmdContext) -> RuntimeResult<String> {
        ctx.token_len_at_least(2)?;
        
        // TODO: make it better, I'm tired for now

        let tokens = ctx.batch.tokens();

        let path_token = &tokens[1];
        let path_string = path_token.value();
        let path = Path::new(&path_string);

        match ctx.env.shell_mut().set_cwd(path) {
            Ok(()) => Ok(String::new()),
            Err(err) => ctx.raise_at_token(
                "CD error has occurred: TODO better",
                path_token
            )?
        }
    } 
}