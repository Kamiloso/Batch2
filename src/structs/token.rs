pub struct Token {
    tok_index: usize,
    value: String,
}

impl Token {
    pub fn tok_index(&self) -> usize { self.tok_index }
    pub fn value(&self) -> &String { &self.value }
    
    pub fn new(tok_index: usize, value: impl Into<String>) -> Self {
        Self {
            tok_index,
            value: value.into(),
        }
    }
    
    pub fn to_value(&self) -> String {
        self.value.clone()
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str { self.value() }
}

impl From<Token> for String {
    fn from(token: Token) -> String { token.to_value() }
}

impl From<&Token> for String {
    fn from(token: &Token) -> String { token.to_value() }
}