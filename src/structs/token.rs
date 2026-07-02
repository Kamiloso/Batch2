use crate::lexer;

pub struct Token {
    tok_index: usize,
    value: String,
}

impl Token {
    pub fn tok_index(&self) -> usize { self.tok_index }
    
    pub fn new(tok_index: usize, value: String) -> Self {
        Self { tok_index, value }
    }
    
    pub fn to_value(&self) -> String {
        self.value.clone()
    }
    
    pub fn to_expanded_value<T>(&self, dict: T) -> String {
        todo!()
    }

    pub fn to_definition(&self) -> String {
        lexer::token_to_definition(&self.value)
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.value
    }
}