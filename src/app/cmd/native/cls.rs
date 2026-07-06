use crate::utils::cli;
use crate::error::RuntimeResult;
use crate::app::cmd::CmdContext;
use crate::define_native_cmd;

define_native_cmd!(Cls, "cls");

impl Cls {
    fn execute(_: CmdContext) -> RuntimeResult<String> {
        cli::clear!();

        Ok(String::new())
    }
}