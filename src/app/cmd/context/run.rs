use crate::app::cmd;
use crate::app::cmd::context::CmdContext;
use crate::error::RuntimeResult;

impl CmdContext<'_, '_> {
    pub fn run_from(&mut self, idx: usize) -> RuntimeResult<()> {
        cmd::delegate_action(self.env, self.stream, &self.batch[idx..])?;
        Ok(())
    }

    pub fn run_from_with(&mut self, idx: usize, wishes: &[(&str, &str)]) -> RuntimeResult<()> {
        let old_memory = wishes
            .iter()
            .map(|&(key, _)| {
                let old = self.env.get(key);
                (key, String::from(old))
            })
            .collect::<Vec<_>>();

        for &(key, value) in wishes {
            self.env.set(key, value);
        }

        let result = self.run_from(idx);

        for (key, value) in old_memory {
            self.env.set(key, &value);
        }

        result
    }
    
    pub fn run_source_code(&mut self, source_code: &str) -> RuntimeResult<()> {
        match self.env.interpret(source_code, &mut self.stream) {
            // TODO: Make error handling more robust
            Err(interpreter) => Err(self.raise_raw("ERROR EXECUTING INTERNAL CODE")?),
            Ok(()) => Ok(()),
        }
    }
}