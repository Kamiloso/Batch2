use std::path::PathBuf;
use clap::{Parser, ArgAction};
use crate::app::environment::Environment;
use crate::app::shell::Shell;

#[derive(Parser)]
#[command(version)]
#[command(disable_version_flag = true)]
struct Cli {
    #[arg(short = 'v', long = "version", action = ArgAction::Version)]
    version: Option<bool>,

    path: Option<String>,
}

pub enum RunMode {
    Terminal { env: Environment },
    Script { env: Environment, path: PathBuf },
}

impl RunMode {
    pub fn obtain() -> Self {
        let cli = Cli::parse();

        match cli.path {
            Some(path_str) => {
                let env = Environment::new(Shell::init());
                let path = PathBuf::from(path_str);
                Self::Script { env, path }
            }
            None => {
                let env = Environment::new(Shell::init());
                Self::Terminal { env }
            }
        }
    }
}