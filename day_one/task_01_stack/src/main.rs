// Step 1: Define the Stack struct
pub struct Stack<T> {
    items: Vec<T>,
}

// Step 2: Implement basic operations
impl<T> Stack<T> {
    // Push operation
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Pop operation
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Peek operation
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // IsEmpty operation
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // Size operation
    pub fn size(&self) -> usize {
        self.items.len()
    }
}

// Step 3: Main function for demonstration
fn main() {
    let mut stack = Stack { items: Vec::new() };

    // Push some elements onto the stack
    stack.push(10);
    stack.push(20);
    stack.push(30);

    // Peek the top element
    if let Some(top) = stack.peek() {
        println!("Top of the stack: {}", top);
    }

    // Pop an element from the stack
    if let Some(popped) = stack.pop() {
        println!("Popped element: {}", popped);
    }

    // Check the size of the stack
    println!("Current stack size: {}", stack.size());

    // Check if the stack is empty
    if stack.is_empty() {
        println!("The stack is empty.");
    } else {
        println!("The stack is not empty.");
    }
}

// Step 4: Testing the Stack (This part stays the same)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack { items: Vec::new() };
        stack.push(1);
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack { items: Vec::new() };
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack { items: Vec::new() };
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn test_is_empty() {
        let mut stack = Stack { items: Vec::new() };
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_size() {
        let mut stack = Stack { items: Vec::new() };
        assert_eq!(stack.size(), 0);
        stack.push(1);
        assert_eq!(stack.size(), 1);
    }
}
