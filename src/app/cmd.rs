mod native;
mod context;

use crate::error::RuntimeResult;
use crate::app::environment::Environment;
use crate::structs::Batch;
use context::CmdContext;
use native::NATIVE_CMD_LIST;

trait Cmd {
    fn name(&self) -> &str;
    fn name_matches(&self, name: &str) -> bool;
    fn execute(&self, ctx: CmdContext) -> RuntimeResult<String>;
}

pub fn delegate_action(env: &mut Environment, batch: &Batch) -> RuntimeResult<String> {
    let ctx = CmdContext::new_mock(env, batch);
    let tokens = batch.tokens();

    if tokens.is_empty() {
        return Ok(String::new());
    }
    
    let name = &ctx.token_expansion_at(0);
    
    for &cmd in NATIVE_CMD_LIST {
        if cmd.name_matches(name) {
            return Ok(cmd.execute(CmdContext::new(cmd, env, batch))?);
        }
    }
    
    ctx.raise_at_token("Unknown command", 0)?
}