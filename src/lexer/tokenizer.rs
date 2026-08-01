use std::iter;
use crate::error::SyntaxResult;
use crate::lexer::builder::LexBuilder;
use crate::lexer::state::{PtrState, FreeMode, PtrReturn};
use crate::lexer::context::SyntaxContext;
use crate::tools::chars;
use crate::structs::BatchBuf;
use crate::utils::utf8;

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
    
    pub fn split_into_batches(mut self) -> SyntaxResult<Vec<BatchBuf>> {
        let mut chars = self.source_code.chars()
            .chain(iter::once(chars::NEWLINE));
        
        while let Some(char_pair) = self.take_unescaped_char(&mut chars)? {
            let CharPair {c, is_escaped} = char_pair;
            
            let Some(PtrReturn {
                new_state,
                is_add_char,
                is_flush_to_batch,
                is_flush_to_vec,
                is_pure,
            }) = self.state.get_return_struct(c, is_escaped) else {
                self.context.raise_at_char("Unexpected character", c)?
            };
            
            self.state = new_state;

            if is_add_char {
                self.builder.add_char(c);
            }
            if is_flush_to_batch {
                self.builder.flush_to_batch(is_pure);
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
    
    fn take_unescaped_char(
        &mut self,
        iter: &mut impl Iterator<Item = char>
    ) -> SyntaxResult<Option<CharPair>> {
        let Some(c0) = self.take_char(iter) else {
            return Ok(None);
        };

        let ctx_snapshot = self.context.clone();
        let escaper = self.state.escape_strategy();
        
        if !escaper.is_escape_char(c0) {
            return Ok(Some(CharPair {
                c: c0,
                is_escaped: false,
            }));
        }

        let mut escape_seq = String::from(c0);
        
        while let Some(cn) = self.take_char(iter) {
            escape_seq.push(cn);

            if let Some(c) = escaper.try_unescape_seq(&escape_seq) {
                return Ok(Some(CharPair {
                    c,
                    is_escaped: true,
                }));
            }
        }

        let trunc = utf8::truncate(&escape_seq, 2);
        ctx_snapshot.raise_at_seq("Invalid escape sequence", trunc)?
    }
    
    fn take_char(&mut self, iter: &mut impl Iterator<Item = char>) -> Option<char> {
        let c = iter.next()?;
        
        self.context.observe_char(c);
        self.builder.observe_char(c);
        
        Some(c)
    }
}