pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    pub fn peek(&mut self) -> Option<&T> {
        self.data.last()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod stack {}
