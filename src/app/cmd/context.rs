mod raise;
mod parse;
mod ensure;
mod token;
mod eval;
mod run;

use crate::app::Environment;
use crate::structs::Batch;
use crate::tools::stream::Stream;
use crate::error::RuntimeResult;
use super::Cmd;
use super::tech::MOCK_CMD;
pub use eval::EvalPair;

pub struct CmdContext<'a, 's> {
    cmd: &'a dyn Cmd,
    env: &'a mut Environment,
    stream: &'a mut Stream<'s>,
    batch: &'a Batch,
}

impl<'a, 's> CmdContext<'a, 's> {
    pub fn env(&self) -> &Environment { &self.env }
    pub fn env_mut(&mut self) -> &mut Environment { &mut self.env }
    pub fn stream(&self) -> &Stream<'s> { &self.stream }
    pub fn stream_mut(&mut self) -> &mut Stream<'s> { &mut self.stream }
    
    pub fn new(
        cmd: &'a dyn Cmd,
        env: &'a mut Environment,
        stream: &'a mut Stream<'s>,
        batch: &'a Batch
    ) -> Self {
        Self { cmd, env, stream, batch }
    }
}