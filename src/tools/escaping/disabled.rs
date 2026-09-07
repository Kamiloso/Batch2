use super::EscapeStrategy;

pub struct DisabledEscapeStrategy;

impl EscapeStrategy for DisabledEscapeStrategy {
    fn is_escape_char(&self, c: char) -> bool {
        false
    }

    fn try_unescape_seq(&self, seq: &str) -> Option<char> {
        None
    }
    
    fn char_to_safe_text(&self, char: char) -> String {
        String::from(char)
    }
}