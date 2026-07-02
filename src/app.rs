mod scope;

use std::path::PathBuf;
use std::process::ExitCode;
use crate::errors::Bt2Error;
use crate::utils::cli;
use scope::Scope;

pub fn run_console() -> ExitCode {
    let mut root_scope = Scope::new_root(&PathBuf::new());
    
    loop {
        let cd_path_display = root_scope.cd_path().display();
        let raw_input = cli::multiline_input!("{cd_path_display}> ");
        
        match root_scope.interpret_code(&raw_input) {
            Ok(_) => {},
            Err(Bt2Error::Syntax(err)) => {
                cli::eprintln!("{err}");
            },
            Err(Bt2Error::Exit(err)) => {
                return ExitCode::from(err.code())
            },
        }
    }
}

pub fn run_file<T>(path: T) -> ExitCode {
    todo!()
}