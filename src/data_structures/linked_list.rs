pub struct Node<T> {
    val: T,
    next_node: Option<Box<T>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl <T> LinkedList<T> {
    pub fn new() -> Self {
        
    }

    pub fn add(&mut self) {
        
    }
} 