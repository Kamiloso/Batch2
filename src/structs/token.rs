use crate::lexer;

pub struct Token {
    tok_index: usize,
    value: String,
}

impl Token {
    pub fn tok_index(&self) -> usize { self.tok_index }
    pub fn value(&self) -> &String { &self.value }
    
    pub fn new(tok_index: usize, value: impl Into<String>) -> Self {
        Self {
            tok_index,
            value: value.into(),
        }
    }
    
    pub fn to_value(&self) -> String {
        self.value.clone()
    }

    pub fn to_definition(&self) -> String {
        lexer::token_to_definition(&self.value)
    }
}