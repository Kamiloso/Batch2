use crate::tools::chars;
use crate::lexer::escaping::RustEscapeStrategy;
use crate::lexer::state::{PtrReturn, PtrState, *};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Before,
    During,
    After,
}

#[derive(Default, Clone)]
pub struct QuoteMode {
    mode: Mode,
}

impl QuoteMode {
    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
}

impl PtrState for QuoteMode {
    fn escape_strategy(&self) -> Box<dyn EscapeStrategy> {
        Box::new(RustEscapeStrategy)
    }
    
    fn get_return_struct(&self, c: char, is_escaped: bool) -> Option<PtrReturn> {
        if chars::is_quote(c) && !is_escaped {
            match self.mode {
                Mode::Before => Some(PtrReturn::new(self.clone().with_mode(Mode::During))),
                Mode::During => Some(PtrReturn::new(self.clone().with_mode(Mode::After))),
                Mode::After => None,
            }
        } else {
            match self.mode {
                Mode::Before => None,
                Mode::During => Some(PtrReturn::new(self.clone()).with_add_char()),
                Mode::After => {
                    if chars::is_transparent(c) {
                        FreeMode::new_fresh().get_return_struct(c, is_escaped)
                    } else {
                        None
                    }
                },
            }
        }
    }
}