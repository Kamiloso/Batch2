use crate::app::cmd::CmdContext;
use crate::error::RuntimeResult;
use crate::define_native_cmd;

define_native_cmd!(Echo, "echo");

impl Echo {
    fn execute(ctx: CmdContext) -> RuntimeResult<String> {
        let mut sb = String::new();
        
        for i in 1..ctx.token_count() {
            sb.push_str(&ctx.token_expansion_at(i));
            sb.push(' ');
        }

        if sb.ends_with(' ') {
            sb.pop();
        }

        Ok(sb + "\n")
    }
}