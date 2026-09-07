use crate::tools::chars::ESCAPE;
use super::EscapeStrategy;
use super::{escape_raw, unescape_raw};

pub struct RustEscapeStrategy;

impl EscapeStrategy for RustEscapeStrategy {
    fn is_escape_char(&self, c: char) -> bool {
        c == ESCAPE
    }

    fn try_unescape_seq(&self, seq: &str) -> Option<char> {
        let text = unescape_raw(seq)?;
        
        if text.chars().count() == 1 {
            text.chars().next()
        } else {
            None
        }
    }

    fn char_to_safe_text(&self, char: char) -> String {
        escape_raw(&String::from(char))
    }
}