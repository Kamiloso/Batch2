use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::define_native_cmd;

define_native_cmd!(Loop, "loop");

impl Loop {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<String> {
        ctx.ensure_args_at_least(2)?;
        
        let repeats = ctx.parse_token::<usize>(1)?;
        let mut out_buffer = String::new();
        
        for _ in 0..repeats {
            out_buffer += &ctx.run_from(2)?;
        }

        Ok(out_buffer)
    }
}