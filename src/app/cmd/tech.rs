use crate::app::cmd::Cmd;

mod mock;
mod interpret;

pub static MOCK_CMD: &(dyn Cmd + Send + Sync) = &mock::Mock;
pub static INTERPRET_CMD: &(dyn Cmd + Send + Sync) = &interpret::Interpret;