use crate::app::shell::Shell;
use crate::app::scope::Scope;

pub struct Environment {
    shell: Shell,
    scopes: Vec<Scope>,
}

impl Environment {
    pub fn shell(&self) -> &Shell { &self.shell }
    pub fn shell_mut(&mut self) -> &mut Shell { &mut self.shell }
    
    pub fn new(shell: Shell) -> Self {
        Self {
            shell,
            scopes: vec![Scope::new()],
        }
    }
    
    pub fn is_root_scope(&self) -> bool {
        self.scopes.len() == 1
    }
    
    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }
    
    pub fn exit_scope(&mut self) {
        if self.is_root_scope() {
            panic!("cannot exit the root scope");
        }
        
        self.scopes.pop();
    }

    pub fn get(&self, key: &str) -> &str {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(key) {
                return value;
            }
        }
        
        ""
    }
    
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.scopes.last_mut().unwrap().set(key, value);
    }
}