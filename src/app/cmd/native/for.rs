use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::types::{ShapeParse, ShapeStringify, ArrayShape, NumberShape, RangeShape};
use super::define_cmd;

define_cmd!(For, "for");

impl For {
    fn execute(mut ctx: CmdContext) -> RuntimeResult<()> {
        ctx.ensure_args_at_least(4)?;
        ctx.ensure_arg_is(2, "in")?;

        let key = &ctx.token_expansion_at(1);
        
        if let Ok((beg, end)) = ctx.parse_token::<RangeShape>(3) {
            for i in beg..end {
                ctx.run_from_with(4, &[
                    (key, &NumberShape::stringify(i))
                ]);
            }

            return Ok(());
        }

        if let Ok(array) = ctx.parse_token::<ArrayShape>(3) {
            for s in array {
                ctx.run_from_with(4, &[
                    (key, &s)
                ]);
            }

            return Ok(());
        }

        ctx.raise_at_token("Iteration subject is invalid", 3)?
    }
}