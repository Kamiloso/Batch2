use crate::app::cmd::Cmd;
use crate::app::cmd::context::CmdContext;
use crate::error::RuntimeResult;

pub struct Mock;

impl Cmd for Mock {
    fn name(&self) -> &str { "__mock__" }
    fn name_matches(&self, _: &str) -> bool { false }

    fn execute(&self, _: CmdContext) -> RuntimeResult<()> {
        Ok(())
    }
}