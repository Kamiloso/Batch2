use std::thread;
use std::time::Duration;
use crate::error::RuntimeResult;
use crate::app::cmd::context::CmdContext;
use super::define_cmd;

define_cmd!(Sleep, "sleep");

impl Sleep {
    fn execute(ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(2)?;

        let time_ms = ctx.parse_token(1)?;
        thread::sleep(Duration::from_millis(time_ms));

        Ok(())
    }
}