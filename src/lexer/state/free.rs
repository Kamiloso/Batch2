use crate::utils::chars;
use crate::lexer::state::{PtrReturn, PtrState, *};
use crate::lexer::escaping::DisabledEscapeStrategy;

#[derive(Clone)]
pub struct FreeMode {
    invoked: bool,
}

impl FreeMode {
    pub fn new_invoked() -> Self {
        Self { invoked: true }
    }
    
    pub fn new_fresh() -> Self {
        Self { invoked: false }
    }
    
    pub fn with_invoked(mut self) -> Self {
        self.invoked = true;
        self
    }
}

impl PtrState for FreeMode {
    fn escape_strategy(&self) -> Box<dyn EscapeStrategy> {
        Box::new(DisabledEscapeStrategy)
    }
    
    fn get_return_struct(&self, c: char, is_escaped: bool) -> Option<PtrReturn> {
        if chars::is_transparent(c) {
            let mut ptr_return = PtrReturn::new(self.clone().with_invoked());

            if !self.invoked {
                ptr_return = ptr_return.with_flush_to_batch();
            }
            
            if chars::is_newline(c) {
                ptr_return = ptr_return.with_flush_to_vec();
            }
            
            return Some(ptr_return);
        }
        
        let next_state: Box<dyn PtrState> = match () {
            _ if chars::is_solid(c) => Box::new(NormalMode::default()),
            _ if chars::is_quote(c) => Box::new(QuoteMode::default()),
            _ if chars::is_colon(c) => Box::new(CommentMode::default()),
            _ => return None
        };
        
        next_state.get_return_struct(c, is_escaped)
    }
}