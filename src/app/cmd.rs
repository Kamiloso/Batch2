mod native;

use std::any;
use std::str::FromStr;
use crate::app::context::RuntimeContext;
use crate::structs::Token;
use crate::error::RuntimeResult;
use crate::lexer;
pub use native::NATIVE_CMDS;

pub trait Cmd {
    fn name(&self) -> &str;
    fn min(&self) -> usize;
    fn execute(&self, context: RuntimeContext) -> RuntimeResult<String>;
}

fn cmd_pre_assert<C: Cmd>(cmd: &impl Cmd, context: &RuntimeContext) -> RuntimeResult<()> {
    let tokens = context.tokens();

    assert!(tokens.len() >= 1);
    assert_eq!(tokens[0].value(), cmd.name());

    if tokens.len() < cmd.min() {
        context.raise_raw(
            format!(
                "Command {} requires at least {} arguments, but only {} was provided",
                lexer::token_to_definition(cmd.name()), cmd.min(), tokens.len()
            )
        )?
    }

    Ok(())
}

fn parse_token<F: FromStr>(context: &RuntimeContext, token: &Token) -> RuntimeResult<F> {
    parse_token_slice::<F>(context, token, token.value())
}

fn parse_token_slice<F: FromStr>(
    context: &RuntimeContext,
    token: &Token,
    s: &str
) -> RuntimeResult<F> {
    match s.parse::<F>() {
        Ok(f) => Ok(f),
        Err(_) => context.raise_at_token(
            format!(
                "Value {} cannot be converted into {}",
                lexer::token_to_definition(s), any::type_name::<F>()
            ),
            token,
        )?
    }
}