pub struct Stack<T> {
    items: Vec<T>,
}

#[derive(Debug)]

// IMPROVE ERROR VARIANTS
pub enum StackError {
    EmptyStack,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: vec![] }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        self.items.pop().ok_or(StackError::EmptyStack)
    }

    pub fn peek(&self) -> Result<&T, StackError> {
        self.items.last().ok_or(StackError::EmptyStack)
    }
}