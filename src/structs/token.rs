#[derive(Clone)]
pub struct Token {
    value: String,
    lin_index: usize,
    tok_index: usize,
}

impl Token {
    pub fn value(&self) -> &str { &self.value }
    pub fn lin_index(&self) -> usize { self.lin_index }
    pub fn tok_index(&self) -> usize { self.tok_index }
    
    pub fn new(value: impl Into<String>, lin_index: usize, tok_index: usize) -> Self {
        Self {
            value: value.into(),
            lin_index, tok_index,
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