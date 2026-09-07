use std::thread;
use std::time::Duration;
use crate::error::RuntimeResult;
use crate::app::cmd::context::CmdContext;
use crate::types::NumberShape;
use super::define_cmd;

define_cmd!(Sleep, "sleep");

impl Sleep {
    fn execute(ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(2)?;

        let time_ms = ctx
            .parse_token::<NumberShape>(1)?
            .clamp(0, i64::MAX) as u64;

        thread::sleep(Duration::from_millis(time_ms));

        Ok(())
    }
}