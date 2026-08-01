mod native;
mod context;
mod tech;

use crate::error::RuntimeResult;
use crate::app::environment::Environment;
use crate::structs::Batch;
use crate::tools::stream::Stream;
use context::CmdContext;
use native::NATIVE_CMD_LIST;
use tech::{MOCK_CMD, INTERPRET_CMD};

trait Cmd {
    fn name(&self) -> &str;
    fn name_matches(&self, name: &str) -> bool;
    fn execute(&self, ctx: CmdContext) -> RuntimeResult<()>;
}

pub fn delegate_action(
    env: &mut Environment,
    stream: &mut Stream,
    batch: &Batch,
) -> RuntimeResult<()> {
    let ctx = CmdContext::new(MOCK_CMD, env, stream, batch);
    let tokens = batch.tokens();

    if tokens.is_empty() {
        return Ok(());
    }
    
    let name = &ctx.token_expansion_at(0);
    
    let available_cmds = NATIVE_CMD_LIST
        .iter()
        .chain(std::iter::once(&INTERPRET_CMD));
    
    for &cmd in available_cmds {
        if cmd.name_matches(name) {
            cmd.execute(CmdContext::new(cmd, env, stream, batch))?;
            return Ok(());
        }
    }

    ctx.raise_at_token("Unknown command", 0)?
}