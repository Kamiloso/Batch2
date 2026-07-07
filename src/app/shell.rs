use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

static INIT_GUARD: AtomicBool = AtomicBool::new(false);

pub struct Shell {
    cd: PathBuf,
}

impl Shell {
    pub fn cd(&self) -> &Path { &self.cd }
    pub fn cd_str(&self) -> &str { &self.cd.to_str().unwrap_or("") }
    
    pub fn init() -> Self {
        if INIT_GUARD.swap(true, Ordering::SeqCst) {
            panic!("shell state initialized multiple times")
        }

        Self {
            cd: obtain_current_dir()
        }
    }

    pub fn set_cd(&mut self, path: &Path) -> io::Result<()> {
        env::set_current_dir(path)?;
        
        self.cd = obtain_current_dir();

        Ok(())
    }
}

fn obtain_current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::new())
}