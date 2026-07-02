mod free;
mod normal;
mod comment;
mod quote;

use crate::lexer::escaping::EscapeStrategy;
pub use free::FreeMode;
pub use normal::NormalMode;
pub use comment::CommentMode;
pub use quote::QuoteMode;

pub trait PtrState
{
    fn escaper(&self) -> Box<dyn EscapeStrategy>;
    fn get_return_struct(&self, c: char, is_escaped: bool) -> Option<PtrReturn>;
}

pub struct PtrReturn {
    pub new_state: Box<dyn PtrState>,
    pub is_add_char: bool,
    pub is_flush_to_batch: bool,
    pub is_flush_to_vec: bool,
}

impl PtrReturn {
    pub fn new(new_state: impl PtrState + 'static) -> Self {
        Self {
            new_state: Box::new(new_state),
            is_add_char: false,
            is_flush_to_batch: false,
            is_flush_to_vec: false,
        }
    }

    pub fn with_add_char(mut self) -> Self {
        self.is_add_char = true;
        self
    }

    pub fn with_flush_to_batch(mut self) -> Self {
        self.is_flush_to_batch = true;
        self
    }

    pub fn with_flush_to_vec(mut self) -> Self {
        self.is_flush_to_vec = true;
        self
    }
}