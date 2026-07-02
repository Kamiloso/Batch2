use crate::structs::Token;
use crate::errors::SyntaxError;

pub struct Batch {
    lin_index: usize,
    tokens: Vec<Token>,
}

impl Batch {
    pub fn lin_index(&self) -> usize { self.lin_index }
    
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
    
    pub fn to_code_snippet(&self) -> String {
        self.tokens.iter()
            .map(Token::to_definition)
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn add_token_by_value(&mut self, value: String) {
        let tok_index = self.tokens.len() + 1;
        let token = Token::new(tok_index, value);
        self.tokens.push(token);
    }

    pub fn syntax_error(&self, msg: &str) -> SyntaxError {
        SyntaxError::new(msg)
            .with_line(self.lin_index)
            .with_code_snippet(&self.to_code_snippet())
    }

    pub fn syntax_error_at_token(&self, msg: &str, token: &Token) -> SyntaxError {
        SyntaxError::new(msg)
            .with_line_and_token(self.lin_index, token.tok_index())
            .with_presentation("Token", &token.to_definition())
            .with_code_snippet(&self.to_code_snippet())
    }
}

impl AsRef<Vec<Token>> for Batch {
    fn as_ref(&self) -> &Vec<Token> {
        &self.tokens
    }
}