use rust_dsa::data_structures::stack::{Stack, StackError};

fn main() {
    let mut my_stack: Stack<i64> = Stack::new();
    my_stack.push(9);
    match my_stack.pop() {
        Ok(value) => println!("Popped: {:?}", value),
        Err(StackError::EmptyStack) => println!("Cannot pop from empty stack"),
    }
    match my_stack.pop() {
        Ok(value) => println!("Popped: {:?}", value),
        Err(StackError::EmptyStack) => println!("Cannot pop from empty stack"),
    }
}
