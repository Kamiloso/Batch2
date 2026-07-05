use crate::structs::Token;

#[derive(Clone)]
pub struct Batch {
    lin_index: usize,
    tokens: Vec<Token>,
}

impl Batch {
    pub fn lin_index(&self) -> usize { self.lin_index }
    pub fn tokens(&self) -> &Vec<Token> { &self.tokens }
    
    pub fn make_first() -> Self {
        Self {
            lin_index: 1,
            tokens: Vec::new(),
        }
    }
    
    pub fn make_next(old_batch: &Batch) -> Self {
        Self {
            lin_index: old_batch.lin_index + 1,
            tokens: Vec::new()
        }
    }

    pub fn add_token_by_value(&mut self, value: impl Into<String>) {
        let tok_index = self.tokens.len() + 1;
        let token = Token::new(tok_index, value);
        self.tokens.push(token);
    }
    
    pub fn to_code_snippet(&self) -> String {
        self.tokens.iter()
            .map(Token::to_definition)
            .collect::<Vec<_>>()
            .join(" ")
    }
}