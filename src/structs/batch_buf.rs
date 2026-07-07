use std::ops::{Deref, Index};
use crate::structs::{Token, Batch};

#[derive(Clone, Default)]
pub struct BatchBuf {
    tokens: Vec<Token>,
}

impl BatchBuf {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    pub fn push_value(&mut self, value: impl Into<String>, lin_index: usize, is_pure: bool) {
        let tok_index = self.tokens.len() + 1;
        let token = Token::new(lin_index, tok_index, is_pure, value);
        
        self.tokens.push(token);
    }
}

impl From<&[Token]> for BatchBuf {
    fn from(slice: &[Token]) -> Self {
        Self { tokens: slice.into() }
    }
}

impl Deref for BatchBuf {
    type Target = Batch;

    fn deref(&self) -> &Batch {
        Batch::from_slice(&self.tokens)
    }
}

impl AsRef<[Token]> for BatchBuf {
    fn as_ref(&self) -> &[Token] {
        &self.tokens
    }
}

impl AsRef<Batch> for BatchBuf {
    fn as_ref(&self) -> &Batch {
        self
    }
}

impl<I> Index<I> for BatchBuf
where
    Batch: Index<I>,
{
    type Output = <Batch as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &(**self)[index]
    }
}