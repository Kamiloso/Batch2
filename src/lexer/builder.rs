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
        self.ensure_empty_word();
        
        let new_batch = Batch::make_next(&self.batch);
        let taken = mem::replace(&mut self.batch, new_batch);
        
        self.vec.push(taken);
    }
    
    pub fn is_vec_ready(&self) -> bool {
        self.word.is_empty() && self.batch.as_ref().is_empty()
    }
    
    pub fn into_vec(self) -> Vec<Batch> {
        self.ensure_empty_word();
        self.ensure_empty_batch();
        
        self.vec
    }
    
    fn ensure_empty_word(&self) {
        if !self.word.is_empty() {
            panic!("word is not empty");
        }
    }
    
    fn ensure_empty_batch(&self) {
        if !self.batch.as_ref().is_empty() {
            panic!("batch is not empty");
        }
    }
}