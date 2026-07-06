use std::env;
use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::sync::atomic::{AtomicBool, Ordering};

static INIT_GUARD: AtomicBool = AtomicBool::new(false);

pub struct Shell {
    cd: Option<PathBuf>,
}

impl Shell {
    pub fn init() -> Self {
        if INIT_GUARD.swap(true, Ordering::SeqCst) {
            panic!("shell state initialized multiple times")
        }

        let cd = match env::current_dir() {
            Ok(dir) => Some(dir),
            Err(_) => None,
        };

        Self { cd }
    }

    pub fn cd(&self) -> Option<&Path> {
        self.cd.as_deref()
    }

    pub fn set_cd(&mut self, path: &Path) -> io::Result<()> {
        env::set_current_dir(path)?;
        
        self.cd = env::current_dir().ok();

        Ok(())
    }

    pub fn cd_display(&self) -> String {
        match &self.cd {
            Some(path) => path.display().to_string(),
            None => format!("?:{}", MAIN_SEPARATOR),
        }
    }
}