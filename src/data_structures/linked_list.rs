pub struct Node<T> {
    val: Option<T>,
    next_node: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
pub enum LinkedListError {
    EmptyList,
    NotFound,
}

impl <T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: Some(Box::new(Node {
                val: None,
                next_node: None,
            })),
        }
    }

    pub fn push_front(&mut self, val: T) {
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Box::new(Node {
            val: Some(val),
            next_node: None,
        });
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.next_node.is_none() {
                node.next_node = Some(new_node);
                return;
            }
            current = &mut node.next_node;
        }
    }

    // result for return
    pub fn pop_front(&mut self) {
        
    }
    // result for return
    pub fn pop_back(&mut self) {

    }

    pub fn insert_after(&mut self, node_val: T, new_val: T) {

    }
    
    // return result
    pub fn remove(&mut self, val: T) {

    }

    pub fn contains(&self, val: T) -> bool {
        true
    }

    pub fn len(&self) -> usize{
        1
    }
    // return result
    pub fn get(&mut self, index: usize) {

    }
    pub fn reverse(&mut self) {

    }
    pub fn clear(&mut self) {

    }
} 

