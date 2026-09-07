use crate::app::cmd::Cmd;
use crate::app::cmd::context::CmdContext;
use crate::error::RuntimeResult;
use crate::types::{ShapeParse, BracketShape};

pub struct Interpret;

impl Cmd for Interpret {
    fn name(&self) -> &str { "__interpret__" }
    fn name_matches(&self, name: &str) -> bool {
        BracketShape::parse(name).is_some()
    }

    fn execute(&self, mut ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(1)?;
        
        let interpret_block = &ctx.token_value_at(0);
        
        if let Some(source_code) = BracketShape::parse(interpret_block) {
            ctx.run_source_code(&source_code)
        } else {
            ctx.raise_at_token("Source code is invalid", 0)?
        }
    }
}