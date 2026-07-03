use std::path::PathBuf;
use std::process::ExitCode;
use bt2::app;

fn main() -> ExitCode {
    app::run_console(PathBuf::new())
}
