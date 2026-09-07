use crate::app::cmd;
use crate::app::shell::Shell;
use crate::app::scope::Scope;
use crate::error::InterpreterResult;
use crate::lexer;
use crate::tools::stream::Stream;

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

    pub fn interpret(&mut self, source_code: &str, stream: &mut Stream) -> InterpreterResult<()> {
        let batches = lexer::compile(source_code)?;

        for batch in batches {
            cmd::delegate_action(self, stream, &batch)?;
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> &str {
        self.scopes.last().unwrap().get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.scopes.last_mut().unwrap().set(key, value);
    }
    
    pub fn get_const(&self, key: &str) -> &str {
        match key.as_ref() {
            "true" => "1",
            "false" => "",
            "cd" => self.shell.cd().to_str().unwrap_or(""),
            _ => "",
        }
    }
    
    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }
    
    pub fn exit_scope(&mut self) {
        if self.scopes.len() == 1 {
            panic!("cannot exit the root scope");
        }
        
        self.scopes.pop();
    }
}