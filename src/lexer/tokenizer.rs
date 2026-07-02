use std::iter;
use crate::errors::{Bt2Result, SyntaxError};
use crate::lexer;
use crate::lexer::builder::LexBuilder;
use crate::lexer::state::{PtrState, FreeMode, PtrReturn};
use crate::lexer::position::LexPosition;
use crate::structs::Batch;
use crate::lexer::chars;

pub struct BatchTokenizer<'a> {
    source_code: &'a str,
    builder: LexBuilder,
    state: Box<dyn PtrState>,
    position: LexPosition,
}

impl<'a> BatchTokenizer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            builder: LexBuilder::new(),
            state: Box::new(FreeMode::new_invoked()),
            position: LexPosition::new(),
        }
    }

    pub fn split_into_batches(mut self) -> Bt2Result<Vec<Batch>> {
        let mut chars = self.source_code.chars()
            .chain(iter::once(chars::NEWLINE));
        
        while let Some((c, is_escaped)) = self.get_next_struct(&mut chars)? {
            let Some(PtrReturn {
                new_state,
                is_add_char,
                is_flush_to_batch,
                is_flush_to_vec,
            }) = self.state.get_return_struct(c, is_escaped) else {
                return self.position.syntax_error("Unexpected character.")
                    .raise();
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

        if self.builder.is_vec_ready() {
            Ok(self.builder.into_vec())
        } else {
            self.position.syntax_error("Unfinished token.")
                .raise()
        }
    }

    fn get_next_struct<T>(&mut self, iter: &mut T) -> Bt2Result<Option<(char, bool)>>
    where
        T: Iterator<Item=char>
    {
        let first = match iter.next() {
            Some(c) => {
                self.position.inject_char(c);
                c
            },
            None => return Ok(None),
        };

        let escaper = self.state.escaper();

        if !escaper.is_escape_char(first) {
            return Ok(Some((first, false)));
        }

        let pos_snapshot = self.position.clone();

        let mut sequence = String::from(first);
        while let Some(c) = iter.next() {
            self.position.inject_char(c);
            sequence.push(c);

            if let Some(c) = escaper.try_unescape_sequence(&sequence) {
                return Ok(Some((c, true)));
            }
        }
        
        pos_snapshot.syntax_error("Invalid escape sequence.")
            .with_presentation("Sequence", &lexer::token_to_definition(&sequence))
            .raise()
    }
}