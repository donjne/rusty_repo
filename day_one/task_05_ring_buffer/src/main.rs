pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buffer: (0..capacity).map(|_| None).collect(),
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.size == self.capacity {
            self.head = (self.head + 1) % self.capacity;
        } else {
            self.size += 1;
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;

        item
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            self.buffer[self.head].as_ref()
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn clear(&mut self) {
        self.buffer = (0..self.capacity).map(|_| None).collect();
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut index = self.head;
        let remaining_size = self.size;
        let buffer = &self.buffer;

        std::iter::repeat_with(move || {
            if remaining_size == 0 {
                return None;
            }

            let item = buffer.get(index).and_then(|opt| opt.as_ref());
            index = (index + 1) % self.capacity;
            item
        })
        .take(remaining_size)
        .flatten() // This ensures the Option<&T> is unwrapped
    }
}

#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn test_push_and_pop() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        assert_eq!(buffer.pop(), Some(1)); // Oldest element
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), None); // Buffer is empty
    }

    #[test]
    fn test_overwrite_when_full() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4); // Overwrites 1

        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), Some(4));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(10);
        assert_eq!(buffer.peek(), Some(&10));
        buffer.push(20);
        assert_eq!(buffer.peek(), Some(&10));
        buffer.pop();
        assert_eq!(buffer.peek(), Some(&20));
    }

    #[test]
    fn test_size_and_is_empty() {
        let mut buffer = RingBuffer::new(2);

        assert!(buffer.is_empty());
        assert_eq!(buffer.size(), 0);

        buffer.push(1);
        buffer.push(2);

        assert!(!buffer.is_empty());
        assert_eq!(buffer.size(), 2);

        buffer.pop();
        assert_eq!(buffer.size(), 1);

        buffer.pop();
        assert_eq!(buffer.size(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_is_full() {
        let mut buffer = RingBuffer::new(2);

        assert!(!buffer.is_full());

        buffer.push(1);
        buffer.push(2);

        assert!(buffer.is_full());

        buffer.pop();
        assert!(!buffer.is_full());
    }

    #[test]
    fn test_clear() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        assert_eq!(buffer.size(), 3);

        buffer.clear();

        assert_eq!(buffer.size(), 0);
        assert_eq!(buffer.pop(), None); // Buffer is empty
    }

    #[test]
    fn test_iter() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(10);
        buffer.push(20);
        buffer.push(30);

        let collected: Vec<_> = buffer.iter().collect();
        assert_eq!(collected, vec![&10, &20, &30]);

        buffer.pop();
        let collected: Vec<_> = buffer.iter().collect();
        assert_eq!(collected, vec![&20, &30]);
    }
}

fn main() {
    // Create a buffer with capacity of 3
    let mut buffer = RingBuffer::new(3);

    // Push some elements into the buffer
    buffer.push(10);
    buffer.push(20);
    buffer.push(30);

    // Print current buffer content by iterating over it
    println!("Buffer content: {:?}", buffer.iter().collect::<Vec<_>>());

    // Push another element, which will overwrite the oldest (10)
    buffer.push(40);
    println!("Buffer after pushing 40: {:?}", buffer.iter().collect::<Vec<_>>());

    // Pop elements and print them
    println!("Popped: {:?}", buffer.pop()); // Should pop 20
    println!("Popped: {:?}", buffer.pop()); // Should pop 30

    // Check remaining elements in the buffer
    println!("Buffer content after pops: {:?}", buffer.iter().collect::<Vec<_>>());

    // Push another element
    buffer.push(50);
    println!("Buffer after pushing 50: {:?}", buffer.iter().collect::<Vec<_>>());

    // Peek the front element
    println!("Peek: {:?}", buffer.peek()); // Should be 40

    // Clear the buffer
    buffer.clear();
    println!("Buffer after clear: {:?}", buffer.iter().collect::<Vec<_>>());
}
