use std::iter;
use crate::error::SyntaxResult;
use crate::lexer::builder::LexBuilder;
use crate::lexer::state::{PtrState, FreeMode, PtrReturn};
use crate::lexer::context::SyntaxContext;
use crate::lexer::chars;
use crate::structs::Batch;
use crate::utils::text;

type SyntaxOption<T> = SyntaxResult<Option<T>>;

struct CharPair {
    c: char,
    is_escaped: bool,
}

pub struct BatchTokenizer<'a> {
    source_code: &'a str,
    builder: LexBuilder,
    state: Box<dyn PtrState>,
    context: SyntaxContext,
}

impl<'a> BatchTokenizer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            builder: LexBuilder::new(),
            state: Box::new(FreeMode::new_invoked()),
            context: SyntaxContext::new(),
        }
    }
    
    pub fn split_into_batches(mut self) -> SyntaxResult<Vec<Batch>> {
        
        let mut chars = self.source_code.chars()
            .chain(iter::once(chars::NEWLINE));
        
        while let Some(char_pair) = self.next_char_pair(&mut chars)? {
            let CharPair { c, is_escaped } = char_pair;
            
            let Some(PtrReturn {
                new_state,
                is_add_char,
                is_flush_to_batch,
                is_flush_to_vec,
            }) = self.state.get_return_struct(c, is_escaped) else {
                self.context.raise_at_char("Unexpected character", c)?
            };
            
            self.state = new_state;

            if is_add_char {
                self.builder.add_char(c);
            }
            if is_flush_to_batch {
                self.builder.flush_to_batch();
            }
            if is_flush_to_vec {
                self.builder.flush_to_vec();
            }
        }
        
        if !self.builder.is_vec_ready() {
            self.context.raise_raw("Unfinished token")?
        }

        Ok(self.builder.into_vec())
    }
    
    fn next_char_pair(&mut self, iter: &mut impl Iterator<Item=char>) -> SyntaxOption<CharPair> {
        let Some(c0) = iter.next() else {
            return Ok(None);
        };
        
        self.context.inject_char(c0);

        let ctx_snapshot = self.context.clone();
        let escaper = self.state.escaper();
        
        if !escaper.is_escape_char(c0) {
            return Ok(Some(CharPair {
                c: c0,
                is_escaped: false
            }));
        }

        let mut escape_seq = String::from(c0);
        
        while let Some(cn) = iter.next() {
            self.context.inject_char(cn);
            escape_seq.push(cn);

            if let Some(c) = escaper.try_unescape_seq(&escape_seq) {
                return Ok(Some(CharPair {
                    c,
                    is_escaped: true,
                }));
            }
        }
        
        ctx_snapshot.raise_at_seq(
            "Invalid escape sequence",
            text::substr(&escape_seq, 0..2)
        )?
    }
}