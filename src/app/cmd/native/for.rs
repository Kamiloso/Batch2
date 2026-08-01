use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::types::{DuckParser, ListType, NumberType, RangeType};
use super::define_cmd;

define_cmd!(For, "for");

impl For {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(4)?;
        ctx.ensure_arg_is(2, "in")?;

        let key = &ctx.token_expansion_at(1);
        let it_subject = &ctx.token_expansion_at(3);
        
        if let Some((beg, end)) = RangeType::parse(it_subject) {
            for i in beg..end {
                ctx.run_from_with(4, &[
                    (key, &NumberType::stringify(i))
                ])?;
            }

            Ok(())
        } else if let Some(n) = NumberType::parse(it_subject) {
            for s in 0..n {
                ctx.run_from_with(4, &[
                    (key, &NumberType::stringify(s))
                ])?;
            }

            Ok(())
        } else if let Some(list) = ListType::parse(it_subject) {
            for s in list {
                ctx.run_from_with(4, &[
                    (key, s)
                ])?;
            }
            
            Ok(())
        } else {
            ctx.raise_at_token("Iteration subject is invalid", 3)?
        }
    }
}