macro_rules! define_chars {
    ($($name:ident, $func:ident, $val:literal);* $(;)?) => {
        $(
            pub const $name: char = $val;
            
            pub fn $func(c: char) -> bool {
                c == $name
            }
        )*
    };
}

define_chars! {
    NEWLINE, is_newline, '\n';
    QUOTE, is_quote, '"';
    COLON, is_colon, ':';
    BRACKET_OPEN, is_bracket_open, '(';
    BRACKET_CLOSE, is_bracket_close, ')';
    PERCENT, is_percent, '%';
    EXCLAMATION, is_exclamation, '!';
}

pub fn is_transparent(c: char) -> bool {
    char::is_whitespace(c) || c == NEWLINE
}

pub fn is_solid(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '%' | '!' | '=' | '.')
}