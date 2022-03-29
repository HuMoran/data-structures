pub struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }
    pub fn add(&mut self, item: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, item);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
