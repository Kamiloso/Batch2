mod disabled;
mod rust;

pub use disabled::DisabledEscapeStrategy;
pub use rust::RustEscapeStrategy;

pub trait EscapeStrategy {
    fn is_escape_char(&self, c: char) -> bool;
    fn try_unescape_sequence(&self, sequence: &str) -> Option<char>;
    fn char_to_safe_text(&self, char: char) -> String;
}

pub fn escape(s: &str) -> String {
    s.escape_default().collect()
}

pub fn unescape(s: &str) -> Option<String> {
    unescape::unescape(s)
}