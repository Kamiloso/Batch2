use crate::app::cmd::Cmd;

mod echo;
mod cls;
mod exit;
mod cd;

#[linkme::distributed_slice]
pub static NATIVE_CMD_LIST: [&(dyn Cmd + Send + Sync)];

#[macro_export]
macro_rules! define_native_cmd {
    ($id:ident, $name:expr) => {
        pub struct $id;

        #[linkme::distributed_slice(crate::app::cmd::native::NATIVE_CMD_LIST)]
        static CMD: &(dyn crate::app::cmd::Cmd + Send + Sync) = &$id;

        impl crate::app::cmd::Cmd for $id {
            fn name(&self) -> &str {
                $name
            }
            
            fn name_matches(&self, name: &str) -> bool {
                name == $name
            }
            
            fn execute(
                &self,
                context: crate::app::cmd::CmdContext
            ) -> crate::error::RuntimeResult<String> {
                $id::execute(context)
            }
        }
    };
}