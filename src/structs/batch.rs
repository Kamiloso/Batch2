use std::ops::Index;
use crate::structs::Token;

#[repr(transparent)]
pub struct Batch {
    tokens: [Token],
}

impl Batch {
    pub fn from_slice(slice: &[Token]) -> &Self {
        // WHY SAFE: Batch has the same memory layout as [Token]
        unsafe { &*(slice as *const [Token] as *const Batch) }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn lin_index(&self) -> usize {
        self.tokens.first().map_or(0, |t| t.lin_index())
    }
}

impl AsRef<[Token]> for Batch {
    fn as_ref(&self) -> &[Token] {
        &self.tokens
    }
}

impl Index<usize> for Batch {
    type Output = Token;

    fn index(&self, index: usize) -> &Token {
        &self.tokens[index]
    }
}

macro_rules! impl_index_for_batch {
    ($($t:ty),*) => {
        $(
            impl Index<$t> for Batch {
                type Output = Batch;

                fn index(&self, index: $t) -> &Batch {
                    Batch::from_slice(&self.tokens[index])
                }
            }
        )*
    }
}

impl_index_for_batch!(
    std::ops::Range<usize>,
    std::ops::RangeFrom<usize>,
    std::ops::RangeTo<usize>,
    std::ops::RangeFull,
    std::ops::RangeInclusive<usize>,
    std::ops::RangeToInclusive<usize>
);