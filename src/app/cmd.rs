mod echo;
mod cls;
mod exit;

use std::any;
use std::str::FromStr;
use crate::app::context::BatchContext;
use crate::structs::Token;
use crate::error::RuntimeResult;
use crate::lexer;
pub use echo::Echo;
pub use cls::Cls;
pub use exit::Exit;

pub trait Cmd {
    fn name() -> &'static str;
    fn min() -> usize;
    fn execute(context: BatchContext) -> RuntimeResult<String>;
}

fn cmd_pre_assert<C: Cmd>(context: &BatchContext) -> RuntimeResult<()> {
    let tokens = context.tokens();

    assert!(tokens.len() >= 1);
    assert_eq!(tokens[0].value(), C::name());

    if tokens.len() < C::min() {
        context.raise_raw(
            format!(
                "Command {} requires at least {} arguments, but only {} was provided",
                lexer::token_to_definition(C::name()), C::min(), tokens.len()
            )
        )?
    }

    Ok(())
}

fn parse_token<F: FromStr>(context: &BatchContext, token: &Token) -> RuntimeResult<F> {
    parse_token_slice::<F>(context, token, token.value())
}

fn parse_token_slice<F: FromStr>(
    context: &BatchContext,
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