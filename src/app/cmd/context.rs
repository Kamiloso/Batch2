mod raise;
mod parse;
mod ensure;
mod mock;
mod token;
mod eval;
mod run;

use crate::app::cmd::Cmd;
use crate::app::cmd::context::mock::MockCmd;
use crate::app::Environment;
use crate::structs::Batch;
pub use eval::EvalPair;

pub struct CmdContext<'a> {
    cmd: &'a dyn Cmd,
    env: &'a mut Environment,
    batch: &'a Batch,
}

impl<'a> CmdContext<'a> {
    pub fn env(&self) -> &Environment { &self.env }
    pub fn env_mut(&mut self) -> &mut Environment { &mut self.env }
    
    pub fn new(cmd: &'a dyn Cmd, env: &'a mut Environment, batch: &'a Batch) -> Self {
        Self { cmd, env, batch }
    }
    
    pub fn new_mock(env: &'a mut Environment, batch: &'a Batch) -> Self {
        Self { cmd: &MockCmd, env, batch }
    }
}