mod disabled;
mod rust;

use crate::tools::chars::QUOTE;
pub use disabled::*;
pub use rust::*;

pub trait EscapeStrategy {
    fn is_escape_char(&self, c: char) -> bool;
    fn try_unescape_seq(&self, seq: &str) -> Option<char>;
    fn char_to_safe_text(&self, char: char) -> String;
}

pub fn escape_raw(s: &str) -> String {
    s.escape_default().collect()
}

pub fn unescape_raw(s: &str) -> Option<String> {
    unescape::unescape(s)
}

pub fn escape_quoted(s: &str) -> String {
    std::iter::once(QUOTE)
        .chain(s.escape_default())
        .chain(std::iter::once(QUOTE))
        .collect()
}

pub fn unescape_quoted(s: &str) -> Option<String> {
    unescape::unescape(s
        .strip_prefix(QUOTE)?
        .strip_suffix(QUOTE)?
    )
}