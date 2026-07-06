use crate::app::cmd::Cmd;
use crate::app::cmd::context::CmdContext;
use crate::error::RuntimeResult;

pub struct MockCmd;

impl Cmd for MockCmd {
    fn name(&self) -> &str { "" }
    fn name_matches(&self, _: &str) -> bool { false }
    
    fn execute(&self, _: CmdContext) -> RuntimeResult<String> {
        Ok(String::new())
    }
}