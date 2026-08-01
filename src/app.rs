mod environment;
mod outcome;
mod cmd;
mod shell;
mod scope;
mod run_mode;

use std::fs;
use std::path::Path;
use std::process::ExitCode;
use crate::utils::cli;
use crate::tools::stream::Stream;
use outcome::Outcome;
use environment::Environment;
use run_mode::RunMode;

pub fn run() -> ExitCode {
    match RunMode::obtain() {
        RunMode::Terminal { env } => run_terminal(env),
        RunMode::Script { env, path } => run_script(env, &path),
    }
}

fn run_terminal(mut env: Environment) -> ExitCode {
    loop {
        let source_code = cli::multiline_input!("{}> ", env.shell().cd_str());
        let mut stream = Stream::console();
        let result = env.interpret(&source_code, &mut stream);

        match Outcome::from(result) {
            Outcome::Continue => { },
            Outcome::Failure { display } => { cli::eprintln!("{display}"); },
            Outcome::Exit { code } => return code,
        };
    }
}

fn run_script(mut env: Environment, path: &Path) -> ExitCode {
    match fs::read_to_string(path) {
        Ok(source_code) => {
            let mut stream = Stream::console();
            let result = env.interpret(&source_code, &mut stream);

            match Outcome::from(result) {
                Outcome::Continue => ExitCode::SUCCESS,
                Outcome::Failure { display } => {
                    cli::eprintln!("{display}");
                    ExitCode::FAILURE
                },
                Outcome::Exit { code } => code,
            }
        },
        Err(err) => {
            cli::eprintln!("Cannot read file: {}", err);
            ExitCode::FAILURE
        },
    }
}