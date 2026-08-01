use crate::utils::cli;
use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use super::define_cmd;

define_cmd!(Cls, "cls");

impl Cls {
    fn execute(_: CmdContext) -> RuntimeResult<()> {
        cli::clear!();

        Ok(())
    }
}