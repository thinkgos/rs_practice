pub struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(cap: usize) -> Self {
        Queue {
            cap: cap,
            data: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&mut self) -> bool {
        self.data.is_empty()
    }
    pub fn enqueue(&mut self, value: T) -> Result<(), String> {
        if self.len() == self.cap {
            Err("No space available".to_string())
        } else {
            self.data.insert(0, value);
            Ok(())
        }
    }
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop()
    }
}
