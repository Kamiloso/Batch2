use crate::app::cmd::Cmd;
use crate::error::RuntimeResult;
use super::CmdContext;

pub struct MockCmd;

impl Cmd for MockCmd {
    fn name(&self) -> &str { "" }
    fn name_matches(&self, _: &str) -> bool { false }
    
    fn execute(&self, _: CmdContext) -> RuntimeResult<String> {
        Ok(String::new())
    }
}