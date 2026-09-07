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
    ESCAPE, is_escape, '\\';
    PLUS, is_plus, '+';
    MINUS, is_minus, '-';
    MULTIPLY, is_multiply, '*';
    DIVIDE, is_divide, '/';
    EQUALS, is_equals, '=';
    DOT, is_dot, '.';
    UNDERSCORE, is_underscore, '_';
}

pub fn is_whitespace(c: char) -> bool {
    char::is_whitespace(c)
}

pub fn is_expansion(c: char) -> bool {
    matches!(c, PERCENT | EXCLAMATION)
}

pub fn is_solid(c: char) -> bool {
    // TODO: '=' is temporary
    c.is_alphanumeric() || is_expansion(c) || matches!(c, UNDERSCORE | DOT | '=')
}

pub fn is_operator(c: char) -> bool {
    matches!(c, PLUS | MINUS | MULTIPLY | DIVIDE | EQUALS | EXCLAMATION)
}