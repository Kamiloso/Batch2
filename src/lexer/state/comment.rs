use crate::utils::chars;
use crate::lexer::escaping::DisabledEscapeStrategy;
use crate::lexer::state::{PtrReturn, PtrState, *};

#[derive(Default, Clone)]
pub struct CommentMode {
    colons: usize,
}

impl CommentMode {
    const REQ_COLONS: usize = 2;
    
    pub fn increment_colons(mut self) -> Self {
        self.colons += 1;
        self
    }
}

impl PtrState for CommentMode {
    fn escape_strategy(&self) -> Box<dyn EscapeStrategy> {
        Box::new(DisabledEscapeStrategy)
    }
    
    fn get_return_struct(&self, c: char, is_escaped: bool) -> Option<PtrReturn> {
        if chars::is_colon(c) {
            return Some(PtrReturn::new(self.clone().increment_colons()));
        }
        
        if chars::is_newline(c) {
            return FreeMode::new_invoked().get_return_struct(c, is_escaped);
        }
        
        if self.colons < Self::REQ_COLONS {
            return None;
        }

        Some(PtrReturn::new(self.clone()))
    }
}