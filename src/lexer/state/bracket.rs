use crate::tools::chars;
use crate::lexer::escaping::{DisabledEscapeStrategy, RustEscapeStrategy};
use crate::lexer::state::{PtrReturn, PtrState, *};

#[derive(Default, Clone)]
pub struct BracketMode {
    level: usize,
}

impl BracketMode {
    pub fn with_level_up(mut self) -> Self {
        self.level += 1;
        self
    }
    
    pub fn with_level_down(mut self) -> Self {
        self.level -= 1;
        self
    }
}

impl PtrState for BracketMode {
    fn escape_strategy(&self) -> Box<dyn EscapeStrategy> {
        Box::new(DisabledEscapeStrategy)
    }
    
    // TODO: include quotes with brackets inside brackets

    fn get_return_struct(&self, c: char, is_escaped: bool) -> Option<PtrReturn> {
        if chars::is_bracket_open(c) {
            return Some(PtrReturn::new(self.clone().with_level_up()).with_add_char());
        }

        if self.level == 0 {
            return None;
        }

        if chars::is_bracket_close(c) {
            return if self.level == 1 {
                Some(PtrReturn::new(FreeMode::new_fresh()).with_add_char())
            } else {
                Some(PtrReturn::new(self.clone().with_level_down()).with_add_char())
            }
        }

        Some(PtrReturn::new(self.clone()).with_add_char())
    }
}