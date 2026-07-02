use super::EscapeStrategy;
use super::{escape, unescape};

const ESCAPE: char = '\\';

pub struct RustEscapeStrategy;

impl EscapeStrategy for RustEscapeStrategy {
    fn is_escape_char(&self, c: char) -> bool {
        c == ESCAPE
    }

    fn try_unescape_sequence(&self, sequence: &str) -> Option<char> {
        let text = unescape(sequence)?;
        if text.len() == 1 {
            text.chars().next()
        } else {
            None
        }
    }

    fn char_to_safe_text(&self, char: char) -> String {
        escape(&String::from(char))
    }
}