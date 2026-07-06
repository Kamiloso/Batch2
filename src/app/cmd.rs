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
    let tokens = batch.tokens();

    if tokens.is_empty() {
        return Ok(String::new());
    }

    let name_token = &tokens[0];
    let name = name_token.value();

    for &cmd in NATIVE_CMD_LIST {
        if cmd.name_matches(name) {
            return Ok(cmd.execute(CmdContext::new(cmd, env, batch))?);
        }
    }
    
    CmdContext::new_mock(env, batch)
        .raise_at_token("Unknown command", name_token)?
}