mod observer;

use crate::structs::BatchBuf;
use observer::LineObserver;

pub struct LexBuilder {
    value: String,
    batch: BatchBuf,
    vec: Vec<BatchBuf>,
    observer: LineObserver,
}

impl LexBuilder {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            batch: BatchBuf::new(),
            vec: Vec::new(),
            observer: LineObserver::new(),
        }
    }

    pub fn observe_char(&mut self, c: char) {
        self.observer.observe_char(c);
    }
    
    pub fn add_char(&mut self, c: char) {
        self.observer.lock();
        self.value.push(c);
    }
    
    pub fn flush_to_batch(&mut self, is_pure: bool) {
        let value = std::mem::take(&mut self.value);
        let lin_index = self.observer.release_and_take();
        
        self.batch.push_value(value, lin_index, is_pure);
    }
    
    pub fn flush_to_vec(&mut self) {
        self.assert_word_empty();
        
        let batch = std::mem::take(&mut self.batch);
        self.vec.push(batch);
    }
    
    pub fn is_vec_ready(&self) -> bool {
        self.value.is_empty() && self.batch.tokens().is_empty()
    }
    
    pub fn into_vec(self) -> Vec<BatchBuf> {
        self.assert_word_empty();
        self.assert_batch_empty();
        
        self.vec
    }
    
    fn assert_word_empty(&self) {
        assert!(self.value.is_empty(), "value must be empty");
    }
    
    fn assert_batch_empty(&self) {
        assert!(self.batch.tokens().is_empty(), "batch must be empty");
    }
}