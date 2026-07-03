use std::mem;
use crate::structs::Batch;

pub struct LexBuilder {
    word: String,
    batch: Batch,
    vec: Vec<Batch>,
}

impl LexBuilder {
    pub fn new() -> Self {
        Self {
            word: String::new(),
            batch: Batch::make_first(),
            vec: Vec::new(),
        }
    }
    
    pub fn add_char(&mut self, c: char) {
        self.word.push(c);
    }
    
    pub fn flush_to_batch(&mut self) {
        let taken = mem::take(&mut self.word);

        self.batch.add_token_by_value(taken);
    }
    
    pub fn flush_to_vec(&mut self) {
        self.assert_word_empty();
        
        let new_batch = Batch::make_next(&self.batch);
        let taken = mem::replace(&mut self.batch, new_batch);
        
        self.vec.push(taken);
    }
    
    pub fn is_vec_ready(&self) -> bool {
        self.word.is_empty() && self.batch.tokens().is_empty()
    }
    
    pub fn into_vec(self) -> Vec<Batch> {
        self.assert_word_empty();
        self.assert_batch_empty();
        
        self.vec
    }
    
    fn assert_word_empty(&self) {
        assert!(self.word.is_empty(), "word must be empty");
    }
    
    fn assert_batch_empty(&self) {
        assert!(self.batch.tokens().is_empty(), "batch must be empty");
    }
}