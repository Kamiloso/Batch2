mod raise;
mod parse;
mod ensure;
mod mock;
mod token;

use crate::app::cmd::Cmd;
use crate::app::cmd::context::mock::MockCmd;
use crate::app::Environment;
use crate::structs::Batch;

pub struct CmdContext<'a> {
    pub cmd: &'a dyn Cmd,
    pub env: &'a mut Environment,
    pub batch: &'a Batch,
}

impl<'a> CmdContext<'a> {
    pub fn new(cmd: &'a dyn Cmd, env: &'a mut Environment, batch: &'a Batch) -> Self {
        Self { cmd, env, batch }
    }
    
    pub fn new_mock(env: &'a mut Environment, batch: &'a Batch) -> Self {
        Self { cmd: &MockCmd, env, batch }
    }
}