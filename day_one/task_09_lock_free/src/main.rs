use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::sync::Arc;
use std::thread;

pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    value: T,
    next: *mut Node<T>, // Pointer to the next node in the stack
}

impl<T> LockFreeStack<T> {
    // Create a new empty stack
    pub fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    // Push an element onto the stack
    pub fn push(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = head;
            }

            // Attempt to atomically update the head to the new node using `compare_exchange`.
            if self.head.compare_exchange(head, new_node, Ordering::Release, Ordering::Acquire).is_ok() {
                break;
            }
        }
    }

    // Pop an element from the stack
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None; // Stack is empty
            }

            // Attempt to atomically set the head to the next node.
            let next = unsafe { (*head).next };
            if self.head.compare_exchange(head, next, Ordering::Release, Ordering::Acquire).is_ok() {
                let boxed_node = unsafe { Box::from_raw(head) };
                return Some(boxed_node.value);
            }
        }
    }
}

impl<T> Clone for LockFreeStack<T> {
    fn clone(&self) -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()), // We can't really clone the stack's contents, so leave it empty.
        }
    }
}

fn main() {
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];

    // Spawn threads to push values to the stack
    for i in 0..10 {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            stack.push(i);
            println!("Pushed: {}", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish pushing
    for handle in handles {
        handle.join().unwrap();
    }

    // Pop elements from the stack
    let mut values = vec![];
    while let Some(value) = stack.pop() {
        values.push(value);
    }

    // Ensure all values are popped
    println!("Popped values: {:?}", values);
    assert_eq!(values.len(), 10);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_push_and_pop() {
        let stack = LockFreeStack::new();

        // Push some values
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Pop the values
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None); // Stack is empty
    }

    #[test]
    fn test_pop_empty_stack() {
        let stack: LockFreeStack<i32> = LockFreeStack::new();

        // Attempt to pop from an empty stack
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_concurrent_push_pop() {
        let stack = Arc::new(LockFreeStack::new());
        let mut handles = vec![];

        // Spawn threads to push values concurrently
        for i in 0..5 {
            let stack = Arc::clone(&stack);
            let handle = std::thread::spawn(move || {
                stack.push(i);
            });
            handles.push(handle);
        }

        // Wait for threads to finish pushing
        for handle in handles {
            handle.join().unwrap();
        }

        // Now pop all the values
        let mut values = vec![];
        while let Some(value) = stack.pop() {
            values.push(value);
        }

        // Ensure the stack is empty
        assert_eq!(values.len(), 5);
        assert_eq!(stack.pop(), None);
    }
}
