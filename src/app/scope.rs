use std::path::{Display, PathBuf};
use crate::app::cmd::{Cmd, Cls, Echo, Exit};
use crate::app::context::BatchContext;
use crate::error::{InterpreterResult, RuntimeResult};
use crate::lexer;
use crate::structs::Batch;

pub struct Scope {
    is_root: bool,
    cd_path: PathBuf,
}

impl Scope {
    pub fn new_root(cd_path: impl Into<PathBuf>) -> Self {
        Scope {
            is_root: true,
            cd_path: cd_path.into(),
        }
    }
    
    pub fn cd_path_display(&self) -> Display<'_> {
        self.cd_path.display()
    }

    pub fn interpret(&mut self, source_code: &str) -> InterpreterResult<String> {
        let batches: Vec<Batch> = lexer::split_source_code(source_code)?;
        let mut out_buffer = String::new();
        
        for batch in batches {
            out_buffer += &self.execute_batch(batch)?;
        }

        Ok(out_buffer)
    }

    pub fn execute_batch(&mut self, batch: Batch) -> RuntimeResult<String> {
        let context = BatchContext::new(self, batch);
        let tokens = context.tokens();

        if tokens.is_empty() {
            return Ok(String::new());
        }

        let cmd_token = &tokens[0];
        let cmd_str = cmd_token.value();

        // --- FUTURE MACRO BEGIN ---

        if cmd_str == Echo::name() {
            return Ok(Echo::execute(context)?);
        }

        if cmd_str == Cls::name() {
            return Ok(Cls::execute(context)?);
        }

        if cmd_str == Exit::name() {
            return Ok(Exit::execute(context)?);
        }

        // --- FUTURE MACRO END ---

        context.raise_raw(format!("Unknown command: {cmd_str}"))?
    }
}