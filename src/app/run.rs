use crate::app::cmd::NATIVE_CMDS;
use crate::app::context::RuntimeContext;
use crate::app::environment::Environment;
use crate::error::{InterpreterResult, RuntimeResult};
use crate::structs::Batch;
use crate::lexer;

pub fn interpret(env: &mut Environment, source_code: &str) -> InterpreterResult<String> {
    let batches: Vec<Batch> = lexer::split_source_code(source_code)?;

    let mut out_buffer = String::new();

    for batch in batches {
        out_buffer += &execute_batch(env, batch)?;
    }

    Ok(out_buffer)
}

fn execute_batch(env: &mut Environment, batch: Batch) -> RuntimeResult<String> {
    let context = RuntimeContext::new(env, batch);
    let tokens = context.tokens();

    if tokens.is_empty() {
        return Ok(String::new());
    }

    let cmd_token = &tokens[0];
    let cmd_str = cmd_token.value();

    for &cmd in NATIVE_CMDS {
        if cmd_str == cmd.name() {
            return Ok(cmd.execute(context)?);
        }
    }

    context.raise_at_token("Unknown command", cmd_token)?
}