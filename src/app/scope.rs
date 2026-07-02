use std::path::{Path, PathBuf};
use crate::errors::Bt2Result;
use crate::utils::cli;
use crate::lexer;
use crate::cmd::Cmd;
use crate::structs::Batch;
use crate::cmd::{Echo, Cls, Exit};

pub struct Scope {
    is_root: bool,
    cd_path: PathBuf,
}

impl Scope {
    pub fn is_root(&self) -> bool { self.is_root }
    pub fn cd_path(&self) -> &Path { &self.cd_path }

    pub fn new_root(cd_path: &Path) -> Self {
        Scope {
            is_root: true,
            cd_path: PathBuf::from(cd_path),
        }
    }

    pub fn interpret_code(&mut self, code: &str) -> Bt2Result<String> {
        let mut result_buffer = String::new();
        let batches: Vec<Batch> = lexer::split_source_code(code)?;
        
        for batch in batches {
            let tokens = batch.as_ref();
            
            if tokens.is_empty() {
                continue;
            }
            
            let prim_token = &tokens[0];
            let prim_str = prim_token.as_ref();
            
            let result_part = match prim_str {
                Echo::NAME => Echo::execute(&batch)?,
                Cls::NAME => Cls::execute(&batch)?,
                Exit::NAME => Exit::execute(&batch)?,
                _ => {
                    batch.syntax_error_at_token("Unknown command.", prim_token)
                        .raise()?
                }
            };

            result_buffer += &result_part;
        }

        if self.is_root {
            cli::print!("{}", result_buffer);
            result_buffer.clear();
        }

        Ok(result_buffer)
    }
}