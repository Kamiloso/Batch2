mod scope;
mod outcome;
mod cmd;
mod context;

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use crate::utils::cli;
use outcome::Outcome;
use scope::Scope;

pub fn run_console(cd_path_init: impl Into<PathBuf>) -> ExitCode {
    let mut root = Scope::new_root(cd_path_init);
    
    loop {
        let cd_path_display = root.cd_path_display();
        let source_code = cli::multiline_input!("{cd_path_display}> ");

        match Outcome::from(root.interpret(&source_code)) {
            Outcome::Continue { out: text } => { cli::print!("{text}"); },
            Outcome::Failure { display } => { cli::eprintln!("{display}"); },
            Outcome::Exit { code } => return code,
        }
    }
}

pub fn run_file(cd_path_init: &Path) -> ExitCode {
    todo!()
}