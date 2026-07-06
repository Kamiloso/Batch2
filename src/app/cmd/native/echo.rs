use crate::app::cmd::CmdContext;
use crate::error::RuntimeResult;
use crate::define_native_cmd;

define_native_cmd!(Echo, "echo");

impl Echo {
    fn execute(ctx: CmdContext) -> RuntimeResult<String> {
        let tokens = ctx.batch.tokens();
        let mut sb = String::new();

        for token in tokens[1..].iter() {
            sb.push_str(token.value());
            sb.push(' ');
        }

        if sb.ends_with(' ') {
            sb.pop();
        }

        Ok(sb + "\n")
    }
}