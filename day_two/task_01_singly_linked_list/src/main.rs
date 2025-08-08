use std::fmt;

// Node: Each element in our chain
// T is a generic type - means it can hold any type of data (i32, String, etc.)
struct Node<T> {
    value: T,                           // The actual data we're storing
    next: Option<Box<Node<T>>>,        // Pointer to next node (None if last node)
}
// Box<Node<T>> puts the node on the heap (necessary for recursive structures)
// Option means it can be Some(node) or None

// LinkedList: Container that manages our chain of nodes
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,        // Points to first node (None if empty list)
}

impl<T> LinkedList<T> {
    // Constructor: Creates empty list
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Push: Adds new element at the front (most efficient for singly linked list)
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,                      // Store the new value
            next: self.head.take(),     // take() moves old head to new node's next
        });
        self.head = Some(new_node);     // New node becomes new head
    }
    // Why front insertion? We only have reference to head, not tail

    // Pop: Removes and returns first element
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {   // take() removes head, gives us ownership
            self.head = node.next;      // Second node becomes new head
            node.value                  // Return the value from removed node
        })
        // Returns None if list was empty
    }

    // Check if the list is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Get length by walking through entire list (TRAVERSAL)
    fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;   // Start at head
        while let Some(node) = current {
            count += 1;
            current = &node.next;       // Move to next node
        }
        count
    }

    // Get element at specific index (TRAVERSAL to specific position)
    fn get(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        for _ in 0..index {             // Walk 'index' steps forward
            match current {
                Some(node) => current = &node.next,
                None => return None,    // Index out of bounds
            }
        }
        current.as_ref().map(|node| &node.value)  // Return reference to value
    }
}

// Display trait: Makes our list printable (TRAVERSAL for printing)
impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        write!(f, "[")?;
        while let Some(node) = current {    // Traverse through each node
            write!(f, "{}", node.value)?;
            current = &node.next;           // Move to next node
            if current.is_some() {          // Add arrow if not last element
                write!(f, " -> ")?;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    let mut list = LinkedList::new();
    
    // Push adds to front, so order reverses
    list.push(1);    // List: [1]
    list.push(2);    // List: [2 -> 1] 
    list.push(3);    // List: [3 -> 2 -> 1]
    
    println!("List: {}", list);         // Prints: [3 -> 2 -> 1]
    println!("Length: {}", list.len()); // Prints: Length: 3
    println!("Is empty: {}", list.is_empty()); // Prints: Is empty: false
    
    // Pop removes from front
    let popped = list.pop();            // Removes 3, list becomes [2 -> 1]
    println!("Popped: {:?}", popped);   // Prints: Popped: Some(3)
    
    // Access element at index 1
    if let Some(value) = list.get(1) {
        println!("Index 1: {}", value); // Prints: Index 1: 1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut list = LinkedList::new();
        
        // Test push and display
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(format!("{}", list), "[3 -> 2 -> 1]");
        
        // Test len
        assert_eq!(list.len(), 3);
        
        // Test pop
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(format!("{}", list), "[1]");
        
        // Test is_empty
        assert!(!list.is_empty());
        assert_eq!(list.pop(), Some(1));
        assert!(list.is_empty());
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_get() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.get(0), Some(&3));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&1));
        assert_eq!(list.get(3), None);
    }
}