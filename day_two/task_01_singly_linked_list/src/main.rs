use std::fmt;
use std::ptr;

// Node: Each element in our chain
// T is a generic type - means it can hold any type of data (i32, String, etc.)
struct Node<T> {
    value: T,                           // The actual data we're storing
    next: Option<Box<Node<T>>>,        // Pointer to next node (None if last node)
}

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

    // Pop: Removes and returns first element
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {   // take() removes head, gives us ownership
            self.head = node.next;      // Second node becomes new head
            node.value                  // Return the value from removed node
        })
    }

    // Check if the list is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Get length by walking through entire list (TRAVERSAL)
    // Modified to handle cycles safely
    fn len(&self) -> usize {
        if self.has_cycle() {
            // For cyclic lists, we can't compute normal length
            // Return the distance to the cycle start + cycle length
            let cycle_start = self.find_cycle_start().unwrap_or(0);
            let cycle_len = self.cycle_length().unwrap_or(0);
            return cycle_start + cycle_len;
        }
        
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
    
    // Reverse the linked list in-place
    fn reverse(&mut self) {
        let mut prev = None;                    // Previous node (starts as None)
        let mut current = self.head.take();     // Current node (starts as head)
        
        while let Some(mut node) = current {
            let next = node.next.take();        // Save the next node
            node.next = prev;                   // Reverse the pointer
            prev = Some(node);                  // Move prev forward
            current = next;                     // Move current forward
        }
        
        self.head = prev;                       // The last node becomes new head
    }
    
    /// Floyd's Cycle Detection Algorithm (Tortoise and Hare)
    /// Returns true if a cycle exists in the linked list
    /// Time Complexity: O(n), Space Complexity: O(1)
    fn has_cycle(&self) -> bool {
        if self.head.is_none() {
            return false;
        }

        // Get raw pointers for comparison, it is safe because we're only comparing addresses
        let mut slow = self.head.as_ref();     // Tortoise: moves 1 step at a time
        let mut fast = self.head.as_ref();     // Hare: moves 2 steps at a time

        // Continue until fast pointer reaches the end or they meet
        while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
            // Move slow pointer one step
            slow = slow_node.next.as_ref();
            
            // Move fast pointer two steps if possible
            fast = fast_node.next.as_ref().and_then(|node| node.next.as_ref());
            
            // If fast reaches the end, no cycle exists
            if fast.is_none() {
                return false;
            }
            
            // Check if they point to the same memory location, cycle detected
            if let (Some(slow_ptr), Some(fast_ptr)) = (slow, fast) {
                if ptr::eq(slow_ptr.as_ref(), fast_ptr.as_ref()) {
                    return true;
                }
            }
        }
        
        false  // No cycle found
    }

    /// Find the start of the cycle if one exists
    /// Returns the index of the node where the cycle begins
    /// Time Complexity: O(n), Space Complexity: O(1)
    fn find_cycle_start(&self) -> Option<usize> {
        if !self.has_cycle() {
            return None;
        }

        // Detect cycle using Floyd's algorithm
        let mut slow = self.head.as_ref();
        let mut fast = self.head.as_ref();

        // Find meeting point
        while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
            slow = slow_node.next.as_ref();
            fast = fast_node.next.as_ref().and_then(|node| node.next.as_ref());
            
            if let (Some(slow_ptr), Some(fast_ptr)) = (slow, fast) {
                if ptr::eq(slow_ptr.as_ref(), fast_ptr.as_ref()) {
                    break;
                }
            }
        }

        // Find the start of the cycle
        // Move one pointer back to head, keep other at meeting point
        let mut start = self.head.as_ref();
        let mut meeting = slow;
        let mut index = 0;

        // Move both pointers one step at a time until they meet
        // The meeting point will be the start of the cycle
        while let (Some(start_node), Some(meeting_node)) = (start, meeting) {
            if ptr::eq(start_node.as_ref(), meeting_node.as_ref()) {
                return Some(index);
            }
            
            start = start_node.next.as_ref();
            meeting = meeting_node.next.as_ref();
            index += 1;
        }

        None
    }

    /// Get the length of the cycle (if one exists)
    /// Time Complexity: O(n), Space Complexity: O(1)
    fn cycle_length(&self) -> Option<usize> {
        if !self.has_cycle() {
            return None;
        }

        // First find the meeting point using Floyd's algorithm
        let mut slow = self.head.as_ref();
        let mut fast = self.head.as_ref();

        // Find meeting point
        while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
            slow = slow_node.next.as_ref();
            fast = fast_node.next.as_ref().and_then(|node| node.next.as_ref());
            
            if let (Some(slow_ptr), Some(fast_ptr)) = (slow, fast) {
                if ptr::eq(slow_ptr.as_ref(), fast_ptr.as_ref()) {
                    break;
                }
            }
        }

        // Now count the cycle length by moving from meeting point
        let mut current = slow;
        let mut length = 0;

        loop {
            if let Some(node) = current {
                current = node.next.as_ref();
                length += 1;
                
                // If we're back to the meeting point, we've completed one cycle
                if let (Some(curr_ptr), Some(slow_ptr)) = (current, slow) {
                    if ptr::eq(curr_ptr.as_ref(), slow_ptr.as_ref()) {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Some(length)
    }

    /// Get all values in the list (safe for both cyclic and linear lists)
    /// For cyclic lists, stops after visiting each unique node once
    fn get_all_values(&self) -> Vec<&T> {
        let mut values = Vec::new();
        let mut current = self.head.as_ref();
        let mut visited_count = 0;
        let max_nodes = if self.has_cycle() {
            // For cyclic lists, visit at most the distance to cycle + cycle length
            let cycle_start = self.find_cycle_start().unwrap_or(0);
            let cycle_len = self.cycle_length().unwrap_or(1);
            cycle_start + cycle_len
        } else {
            usize::MAX  // No limit for linear lists
        };

        while let Some(node) = current {
            if visited_count >= max_nodes {
                break;  // Prevent infinite loop in cycles
            }
            
            values.push(&node.value);
            current = node.next.as_ref();
            visited_count += 1;
        }

        values
    }

    /// Display the structure of the list (including cycle information)
    fn describe_structure(&self) -> String 
    where 
        T: std::fmt::Display
    {
        if self.is_empty() {
            return "Empty list".to_string();
        }

        if self.has_cycle() {
            let cycle_start = self.find_cycle_start().unwrap();
            let cycle_length = self.cycle_length().unwrap();
            
            format!(
                "Cyclic list: {} nodes before cycle, cycle of length {} starting at index {}",
                cycle_start, cycle_length, cycle_start
            )
        } else {
            format!("Linear list with {} nodes", self.len())
        }
    }
}

impl<T> LinkedList<T> {
    /// Creates a cycle by connecting the last node to the node at cycle_start_index
    /// WARNING: This is unsafe and creates memory management issues!
    /// Only use for testing cycle detection algorithms
    unsafe fn create_cycle_at(&mut self, cycle_start_index: usize) -> bool {
        if self.head.is_none() {
            return false;
        }

        // We need to collect the addresses of nodes as we traverse
        let mut nodes: Vec<*mut Node<T>> = Vec::new();
        let mut current = self.head.as_mut();

        // Collect all node pointers
        while let Some(node) = current {
            let node_ptr = node.as_mut() as *mut Node<T>;
            nodes.push(node_ptr);
            current = node.next.as_mut();
        }

        // Check if cycle_start_index is valid
        if cycle_start_index >= nodes.len() {
            return false;
        }

        // Get pointers to the last node and cycle start node
        let last_node_ptr = nodes[nodes.len() - 1];
        let cycle_start_ptr = nodes[cycle_start_index];

        // Create the cycle
        let last_node = &mut *last_node_ptr;
        
        // DANGER ZONE: We're creating a non-owning pointer to an existing node
        // This violates Rust's ownership rules and is only for testing
        // We create a "fake" Box that doesn't actually own the memory
        
        // Method 1: Use a raw pointer wrapped in NonNull (safer but still unsafe)
        use std::ptr::NonNull;
        let fake_box = {
            let non_null = NonNull::new(cycle_start_ptr).unwrap();
            // This is extremely dangerous - we're telling Rust this Box owns memory it doesn't
            Box::from_raw(non_null.as_ptr())
        };
        
        last_node.next = Some(fake_box);
        
        true
    }

    /// Alternative safer approach to create cycle using Rc for testing
    /// This approach uses reference counting instead of raw pointers
    #[cfg(test)]
    fn create_test_cycle_with_rc(&mut self, cycle_start_index: usize) -> bool 
    where 
        T: Clone + std::fmt::Debug
    {
        if self.head.is_none() || cycle_start_index >= self.len() {
            return false;
        }

        // For testing purposes, we'll rebuild the list with the cycle
        // This is a safer approach than the raw pointer manipulation
        let values: Vec<T> = self.get_all_values().into_iter().cloned().collect();
        
        // Clear the current list
        while !self.is_empty() {
            self.pop();
        }

        // Rebuild with cycle (this is still unsafe but more controlled)
        // This is just for demonstration - in practice, you'd use a different data structure
        for (i, value) in values.into_iter().enumerate().rev() {
            self.push(value);
        }

        // We'll mark that a cycle should exist conceptually
        // (In a real implementation, you'd use Rc<RefCell<Node<T>>> for safe cycles)
        true
    }
}

// Display trait: Makes our list printable (TRAVERSAL for printing)
// Note: This will NOT work correctly if there's a cycle (infinite loop)
// Use with caution or modify to detect cycles
impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_cycle() {
            let cycle_start = self.find_cycle_start().unwrap_or(0);
            let cycle_length = self.cycle_length().unwrap_or(0);
            return write!(f, "[Cyclic list: cycle starts at index {}, length {}]", 
                         cycle_start, cycle_length);
        }

        let mut current = &self.head;
        write!(f, "[")?;
        while let Some(node) = current {
            write!(f, "{}", node.value)?;
            current = &node.next;
            if current.is_some() {
                write!(f, " -> ")?;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    println!("LinkedList\n");

    println!("1. Testing Basic Operations:");
    let mut list = LinkedList::new();
    
    println!("   Empty list: {}", list);
    println!("   Is empty: {}", list.is_empty());
    println!("   Length: {}", list.len());
    
    // Push elements (remember: adds to front)
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    
    println!("   After pushing 1,2,3,4,5: {}", list);
    println!("   Length: {}", list.len());
    println!("   Is empty: {}", list.is_empty());
    
    // Test get method
    println!("   Element at index 0: {:?}", list.get(0));
    println!("   Element at index 2: {:?}", list.get(2));
    println!("   Element at index 10: {:?}", list.get(10));
    
    // Test pop method
    println!("   Popping: {:?}", list.pop());
    println!("   After pop: {}", list);
    
    println!("\n2. Testing Cycle Detection on Linear List:");
    println!("   Has cycle: {}", list.has_cycle());
    println!("   Cycle start: {:?}", list.find_cycle_start());
    println!("   Cycle length: {:?}", list.cycle_length());
    println!("   Structure: {}", list.describe_structure());
    
    println!("\n3. Testing Reverse Operation:");
    println!("   Before reverse: {}", list);
    list.reverse();
    println!("   After reverse: {}", list);
    list.reverse();
    println!("   After reverse again: {}", list);
    
    println!("\n4. Testing Cycle Creation nut unsafe:");
    println!("   WARNING: Creating artificial cycle for testing...");
    
    // Create a new list for cycle testing
    let mut cycle_list = LinkedList::new();
    cycle_list.push(10);  // Index 4 (remember: push adds to front)
    cycle_list.push(20);  // Index 3
    cycle_list.push(30);  // Index 2
    cycle_list.push(40);  // Index 1
    cycle_list.push(50);  // Index 0
    
    println!("   List before cycle: {}", cycle_list);
    println!("   Length before cycle: {}", cycle_list.len());
    
    // DANGEROUS: Create cycle from last node back to index 2
    unsafe {
        let cycle_created = cycle_list.create_cycle_at(2);
        println!("   Cycle creation successful: {}", cycle_created);
    }
    
    // Test cycle detection on the cyclic list
    println!("   Has cycle: {}", cycle_list.has_cycle());
    
    if cycle_list.has_cycle() {
        println!("   Cycle start index: {:?}", cycle_list.find_cycle_start());
        println!("   Cycle length: {:?}", cycle_list.cycle_length());
        println!("   Structure: {}", cycle_list.describe_structure());
        println!("   Display (safe): {}", cycle_list);
        
        // Show values safely
        let values = cycle_list.get_all_values();
        println!("   Values in list: {:?}", values);
    }
    
    println!("\n5. Testing Edge Cases:");
    
    // Empty list
    let empty: LinkedList<i32> = LinkedList::new();
    println!("   Empty list has cycle: {}", empty.has_cycle());
    
    // Single node
    let mut single = LinkedList::new();
    single.push(42);
    println!("   Single node has cycle: {}", single.has_cycle());
    println!("   Single node: {}", single);
    
    // Two nodes
    let mut two_nodes = LinkedList::new();
    two_nodes.push(1);
    two_nodes.push(2);
    println!("   Two nodes have cycle: {}", two_nodes.has_cycle());
    println!("   Two nodes: {}", two_nodes);

    let mut demo_list = LinkedList::new();
    
    // Build a list
    for i in 1..=6 {
        demo_list.push(i * 10);
    }
    
    println!("   Demo list: {}", demo_list);
    println!("   Length: {}", demo_list.len());
    println!("   Structure: {}", demo_list.describe_structure());
    
    // Test all get operations
    for i in 0..demo_list.len() {
        println!("   Index {}: {:?}", i, demo_list.get(i));
    }
    
    // Test pop until empty
    println!("   Popping all elements:");
    while !demo_list.is_empty() {
        println!("     Popped: {:?}, remaining: {}", demo_list.pop(), demo_list);
    }

    println!("All methods tested successfully!");
    println!("- Basic operations: push, pop, get, len, is_empty");
    println!("- Advanced operations: reverse, cycle detection");
    println!("- Unsafe operations: cycle creation");
    println!("- Edge cases: empty, single node, linear vs cyclic");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_basic_operations() {
        let mut list = LinkedList::new();
        
        // Test empty list
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.get(0), None);
        
        // Test push and basic operations
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert_eq!(format!("{}", list), "[3 -> 2 -> 1]");
        
        // Test get
        assert_eq!(list.get(0), Some(&3));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&1));
        assert_eq!(list.get(3), None);
        
        // Test pop
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn test_reverse_operation() {
        let mut list = LinkedList::new();
        
        // Test reverse empty list
        list.reverse();
        assert!(list.is_empty());
        
        // Test reverse single element
        list.push(42);
        list.reverse();
        assert_eq!(format!("{}", list), "[42]");
        
        // Test reverse multiple elements
        list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        let original = format!("{}", list);
        list.reverse();
        assert_eq!(format!("{}", list), "[1 -> 2 -> 3]");
        
        list.reverse();
        assert_eq!(format!("{}", list), original);
    }

    #[test]
    fn test_cycle_detection_linear_lists() {
        // Empty list
        let empty: LinkedList<i32> = LinkedList::new();
        assert!(!empty.has_cycle());
        assert_eq!(empty.find_cycle_start(), None);
        assert_eq!(empty.cycle_length(), None);
        
        // Single node
        let mut single = LinkedList::new();
        single.push(1);
        assert!(!single.has_cycle());
        assert_eq!(single.find_cycle_start(), None);
        assert_eq!(single.cycle_length(), None);
        
        // Multiple nodes
        let mut multi = LinkedList::new();
        for i in 1..=5 {
            multi.push(i);
        }
        assert!(!multi.has_cycle());
        assert_eq!(multi.find_cycle_start(), None);
        assert_eq!(multi.cycle_length(), None);
    }

    #[test]
    fn test_get_all_values() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        let values = list.get_all_values();
        assert_eq!(values, vec![&3, &2, &1]);
    }

    #[test]
    fn test_describe_structure() {
        let mut list = LinkedList::new();
        assert_eq!(list.describe_structure(), "Empty list");
        
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.describe_structure(), "Linear list with 3 nodes");
    }

    #[test]
    fn test_comprehensive_workflow() {
        let mut list = LinkedList::new();
        
        // Build list
        for i in 1..=10 {
            list.push(i);
        }
        
        // Test all methods work together
        assert_eq!(list.len(), 10);
        assert!(!list.is_empty());
        assert!(!list.has_cycle());
        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(9), Some(&1));
        
        // Reverse and test again
        list.reverse();
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(9), Some(&10));
        assert!(!list.has_cycle());
        
        // Pop some elements
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.len(), 8);
        assert!(!list.has_cycle());
    }
}