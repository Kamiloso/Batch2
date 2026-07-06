mod environment;
mod outcome;
mod cmd;
mod shell;
mod scope;

use std::path::Path;
use std::process::ExitCode;
use crate::utils::cli;
use crate::app::shell::Shell;
use outcome::Outcome;
use environment::Environment;

pub fn run() -> ExitCode {
    let shell = Shell::init();
    
    run_console(shell)
}

fn run_console(shell: Shell) -> ExitCode {
    let mut env = Environment::new(shell);

    loop {
        let prefix = env.shell().cwd_display();
        let source_code = cli::multiline_input!("{prefix}> ");

        let result = env.interpret(&source_code);

        match Outcome::from(result) {
            Outcome::Continue { out: text } => { cli::print!("{text}"); },
            Outcome::Failure { display } => { cli::eprintln!("{display}"); },
            Outcome::Exit { code } => return code,
        }
    }
}

fn run_script(shell: Shell, file: &Path) -> ExitCode {
    todo!()
}