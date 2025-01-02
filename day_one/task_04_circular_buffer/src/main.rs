struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T: Default> CircularBuffer<T> {
    /// Creates a new `CircularBuffer` with the given capacity.
    fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: (0..capacity).map(|_| None).collect(),
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    /// Adds an element to the buffer. If the buffer is full, the oldest element is overwritten.
    fn push(&mut self, item: T) {
        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;

        if self.size == self.capacity {
            self.head = (self.head + 1) % self.capacity; // Move head when overwriting
        } else {
            self.size += 1;
        }
    }

    /// Removes and returns the oldest element from the buffer, or `None` if the buffer is empty.
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let result = self.buffer[self.head].take();
            self.head = (self.head + 1) % self.capacity;
            self.size -= 1;
            result
        }
    }

    /// Checks if the buffer is empty.
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the current number of elements in the buffer.
    fn size(&self) -> usize {
        self.size
    }
}

fn main() {
    let mut cb = CircularBuffer::<i32>::new(3);

    cb.push(1);
    cb.push(2);
    cb.push(3); // Buffer is now full: [1, 2, 3]
    println!("Popped: {:?}", cb.pop()); // Should print Some(1)
    println!("Popped: {:?}", cb.pop()); // Should print Some(2)
    cb.push(4); // Buffer now: [None, None, 4]
    println!("Buffer size: {}", cb.size());
}

#[cfg(test)]
mod tests {
    use super::CircularBuffer;

    #[test]
fn test_circular_buffer_happy_cases() {
    let mut cb = CircularBuffer::<i32>::new(3);

    // Push to empty buffer
    cb.push(1);
    assert_eq!(cb.size(), 1);

    // Basic push and pop
    cb.push(2);
    cb.push(3); // Buffer full
    assert_eq!(cb.pop(), Some(1)); // Oldest is removed
    assert_eq!(cb.size(), 2);

    // Check buffer full behavior
    cb.push(4); // Overwrites the position of 1
    assert_eq!(cb.pop(), Some(2)); // The next oldest element
    assert_eq!(cb.pop(), Some(3)); 
    assert_eq!(cb.pop(), Some(4)); // All elements popped
    assert_eq!(cb.pop(), None); // Buffer now empty

    // Empty check
    assert!(cb.is_empty());
}

#[test]
fn test_circular_buffer_unhappy_cases() {
    let mut cb = CircularBuffer::<i32>::new(2);

    // Pop from empty
    assert_eq!(cb.pop(), None);

    // Push and overwrite
    cb.push(1);
    cb.push(2); // Buffer full
    cb.push(3); // Overwrites 1, now buffer: [3, 2]
    assert_eq!(cb.pop(), Some(2)); // Oldest (not overwritten)
    assert_eq!(cb.pop(), Some(3));
    assert_eq!(cb.pop(), None); // Buffer empty again

    // Multiple operations
    cb.push(4);
    cb.push(5);
    cb.push(6); // Overwrites 4, now buffer: [6, 5]
    assert_eq!(cb.pop(), Some(5)); // Oldest
    cb.push(7); // Buffer: [7, 6]
    assert_eq!(cb.pop(), Some(6));
    assert_eq!(cb.pop(), Some(7));
    assert_eq!(cb.pop(), None);

    // Large buffer test
    let mut large_cb = CircularBuffer::<i32>::new(1000);
    for i in 0..1000 {
        large_cb.push(i);
    }
    for i in 1000 - large_cb.capacity..1000 {
        assert_eq!(large_cb.pop(), Some(i as i32));
    }
}
}