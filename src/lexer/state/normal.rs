use crate::lexer::chars;
use crate::lexer::escaping::DisabledEscapeStrategy;
use crate::lexer::state::{PtrReturn, PtrState, *};

#[derive(Default, Clone)]
pub struct NormalMode;

impl PtrState for NormalMode {
    fn escaper(&self) -> Box<dyn EscapeStrategy> {
        Box::new(DisabledEscapeStrategy)
    }
    
    fn get_return_struct(&self, c: char, is_escaped: bool) -> Option<PtrReturn> {
        if chars::is_solid(c) {
            return Some(PtrReturn::new(self.clone()).with_add_char());
        }
        
        if chars::is_transparent(c) {
            return FreeMode::new_fresh().get_return_struct(c, is_escaped);
        }
        
        None
    }
}