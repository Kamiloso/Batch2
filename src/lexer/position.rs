use crate::errors::SyntaxError;
use crate::lexer;
use crate::lexer::chars;

#[derive(Clone)]
pub struct LexPosition {
    at_char: Option<char>,
    lin_index: usize,
    col_index: usize,
}

impl LexPosition {
    pub fn new() -> Self {
        Self {
            at_char: None,
            lin_index: 1,
            col_index: 0,
        }
    }
    
    pub fn inject_char(&mut self, c: char) {
        self.col_index += 1;
        
        if let Some(value) = self.at_char {
            if chars::is_newline(value) {
                self.lin_index += 1;
                self.col_index = 1;
            }
        }

        self.at_char = Some(c);
    }
    
    pub fn syntax_error(&self, msg: &str) -> SyntaxError {
        SyntaxError::new(msg)
            .with_line_and_column(self.lin_index, self.col_index)
            .with_presentation("Character", &lexer::token_to_definition(
                &String::from(self.at_char.expect("no char injected"))
            ))
    }
}