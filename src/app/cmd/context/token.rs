use crate::app::environment::Environment;
use crate::structs::Token;
use crate::tools::expansion;
use super::CmdContext;

impl CmdContext<'_, '_> {
    pub fn token_at(&self, idx: usize) -> &Token {
        &self.batch.tokens()[idx]
    }
    
    pub fn token_value_at(&self, idx: usize) -> String {
        self.token_at(idx).to_value()
    }
    
    pub fn token_expansion_at(&self, idx: usize) -> String {
        let raw_value = self.token_value_at(idx);
        
        fn get<'a>(env: &'a Environment, key: &str) -> &'a str {
            env.get(key)
        }

        fn get_const<'a>(env: &'a Environment, key: &str) -> &'a str {
            env.get_const(key)
        }

        expansion::perform_expansion(
            raw_value,
            |key| get(self.env, key),
            |key| get_const(self.env, key),
        )
    }

    pub fn tokens(&self) -> &[Token] {
        self.batch.tokens()
    }
    
    pub fn tokens_from(&self, idx: usize) -> &[Token] {
        if idx <= self.tokens_len() {
            &self.tokens()[idx..]
        } else {
            &[]
        }
    }

    pub fn tokens_len(&self) -> usize {
        self.batch.tokens().len()
    }
}