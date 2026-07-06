pub const NEWLINE: char = '\n';
pub const QUOTE: char = '"';
pub const COLON: char = ':';
pub const BRACKET_OPEN: char = '(';
pub const BRACKET_CLOSE: char = ')';

pub fn is_transparent(c: char) -> bool {
    char::is_whitespace(c) || c == NEWLINE
}

pub fn is_newline(c: char) -> bool {
    c == NEWLINE
}

pub fn is_solid(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '%' | '!' | '=')
}

pub fn is_quote(c: char) -> bool {
    c == QUOTE
}

pub fn is_colon(c: char) -> bool {
    c == COLON
}

pub fn is_bracket_open(c: char) -> bool {
    c == BRACKET_OPEN
}

pub fn is_bracket_close(c: char) -> bool {
    c == BRACKET_CLOSE
}