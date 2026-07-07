use crate::app::cmd;
use crate::app::cmd::context::CmdContext;
use crate::error::RuntimeResult;

impl CmdContext<'_> {
    pub fn run_from(&mut self, idx: usize) -> RuntimeResult<String> {
        cmd::delegate_action(self.env, &self.batch[idx..])
    }
}