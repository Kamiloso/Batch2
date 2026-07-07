pub struct LineObserver {
    lin_index_old: usize,
    lin_index_new: usize,
    locked: bool,
}

impl LineObserver {
    pub fn new() -> LineObserver {
        Self {
            lin_index_old: 1,
            lin_index_new: 1,
            locked: false,
        }
    }
    
    pub fn observe_char(&mut self, c: char) {
		self.lin_index_new += 1;
        
        if !self.locked {
            self.lin_index_old += 1;
        }
    }
    
    pub fn lock(&mut self) {
        self.locked = true;
    }
    
    pub fn release_and_take(&mut self) -> usize {
        self.locked = false;
        std::mem::replace(&mut self.lin_index_old, self.lin_index_new)
    }
}