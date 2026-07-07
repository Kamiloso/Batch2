#[derive(Clone)]
pub struct Token {
    lin_index: usize,
    tok_index: usize,
    is_pure: bool,
    value: String,
}

impl Token {
    pub fn lin_index(&self) -> usize { self.lin_index }
    pub fn tok_index(&self) -> usize { self.tok_index }
    pub fn is_pure(&self) -> bool { self.is_pure }
    pub fn value(&self) -> &str { &self.value }
    
    pub fn new(
        lin_index: usize,
        tok_index: usize,
        is_pure: bool,
        value: impl Into<String>,
    ) -> Self {
        Self {
            lin_index, tok_index, is_pure,
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