use std::env;
use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::sync::atomic::{AtomicBool, Ordering};

static INIT_GUARD: AtomicBool = AtomicBool::new(false);

pub struct Shell {
    cwd: Option<PathBuf>,
}

impl Shell {
    pub fn init() -> Self {
        if INIT_GUARD.swap(true, Ordering::SeqCst) {
            panic!("shell state initialized multiple times")
        }

        let cwd = match env::current_dir() {
            Ok(dir) => Some(dir),
            Err(_) => None,
        };

        Self { cwd }
    }

    pub fn cwd(&self) -> Option<&Path> {
        self.cwd.as_deref()
    }

    pub fn set_cwd(&mut self, path: &Path) -> io::Result<()> {
        env::set_current_dir(path)?;
        
        self.cwd = env::current_dir().ok();

        Ok(())
    }

    pub fn cwd_display(&self) -> String {
        match &self.cwd {
            Some(path) => path.display().to_string(),
            None => format!("?:{}", MAIN_SEPARATOR),
        }
    }
}