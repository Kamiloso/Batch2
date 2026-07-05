use crate::app::cmd::Cmd;

mod echo;
mod cls;
mod exit;
mod cd;

pub const NATIVE_CMDS: &[&dyn Cmd] = &[
    &echo::Echo,
    &cls::Cls,
    &exit::Exit,
    &cd::Cd,
];