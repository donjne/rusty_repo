struct Queue<T> {
    enqueue_stack: Vec<T>,
    dequeue_stack: Vec<T>,
}

impl<T> Queue<T> {
    /// Creates an empty queue
    fn new() -> Self {
        Queue {
            enqueue_stack: Vec::new(),
            dequeue_stack: Vec::new(),
        }
    }

    /// Adds an element to the back of the queue
    fn enqueue(&mut self, item: T) {
        self.enqueue_stack.push(item);
    }

    /// Removes an element from the front of the queue if available
    fn dequeue(&mut self) -> Option<T> {
        if self.dequeue_stack.is_empty() {
            // Transfer elements if dequeue_stack is empty
            while let Some(item) = self.enqueue_stack.pop() {
                self.dequeue_stack.push(item);
            }
        }
        self.dequeue_stack.pop()
    }

    /// Returns the number of elements in the queue
    fn size(&self) -> usize {
        self.enqueue_stack.len() + self.dequeue_stack.len()
    }

    /// Checks if the queue is empty
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    println!("Dequeued: {:?}", queue.dequeue());
    println!("Dequeued: {:?}", queue.dequeue());
    println!("Is queue empty? {}", queue.is_empty());
    queue.enqueue(3);
    println!("Queue size: {}", queue.size());
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_operations() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);

        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.size(), 2);
        assert!(!queue.is_empty());

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.size(), 0);
        assert!(queue.is_empty());

        // Test enqueue and dequeue after emptying
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None); // Queue should be empty now
    }
}