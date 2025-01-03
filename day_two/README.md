# Day 2: Linked Lists and Advanced Data Structures

## Objective

On Day 2, we focus on implementing and enhancing linked lists, followed by delving into advanced data structures designed for concurrency, persistence, and optimization. These tasks are aimed at deepening your understanding of Rust's capabilities in building efficient, scalable, and safe systems.

## Tasks Overview

1. **Implement a Singly Linked List**  
2. **Add Reverse Operation to Linked List**  
3. **Detect Cycle in Linked List**  
4. **Merge Two Sorted Linked Lists**  
5. **Implement a Doubly Linked List**  
6. **Implement a Skip List**  
7. **Create a Lock-free Linked List**  
8. **Lock-free Concurrent Skip List**  
9. **Wait-free Queue Implementation**  
10. **Copy-on-write B+ Tree**  
11. **Lock-free Hash Map**  
12. **Persistent Red-Black Tree**  
13. **Concurrent Bitmap Index**  
14. **Cache-oblivious B-tree**  
15. **Write-optimized LSM Tree**

---

## Detailed Task Descriptions

### 1. Implement a Singly Linked List

- **Objective**: Build a singly linked list with basic operations like insertion, deletion, and traversal.  
- **Implementation**:  
  - Define a `Node` struct containing a value and a pointer to the next node.
  - Create a `LinkedList` struct to manage the head node.
  - Implement methods like `push`, `pop`, and `display`.

---

### 2. Add Reverse Operation to Linked List

- **Objective**: Implement an in-place reversal operation for the singly linked list.  
- **Implementation**:  
  - Traverse the list while reversing the `next` pointers.
  - Ensure the head points to the last node after reversal.

---

### 3. Detect Cycle in Linked List

- **Objective**: Identify if a cycle exists in a linked list using efficient algorithms.  
- **Implementation**:  
  - Use Floyd's Cycle Detection Algorithm (Tortoise and Hare) to detect cycles in O(n) time.

---

### 4. Merge Two Sorted Linked Lists

- **Objective**: Merge two sorted singly linked lists into a single sorted linked list.  
- **Implementation**:  
  - Traverse both lists while comparing elements and building the result list.
  - Handle edge cases such as empty lists.

---

### 5. Implement a Doubly Linked List

- **Objective**: Extend the singly linked list to include backward traversal using a doubly linked list.  
- **Implementation**:  
  - Add a `prev` pointer to the `Node` struct.
  - Implement methods for insertion and deletion at both ends.

---

### 6. Implement a Skip List

- **Objective**: Build a skip list for faster search operations, balancing simplicity and efficiency.  
- **Implementation**:  
  - Use multiple levels of linked lists with different step sizes.
  - Implement search, insertion, and deletion methods.

---

### 7. Create a Lock-free Linked List

- **Objective**: Design a thread-safe, lock-free linked list for concurrent environments.  
- **Implementation**:  
  - Use atomic operations to manage node insertion and deletion.
  - Ensure memory safety with Rust's ownership model.

---

### 8. Lock-free Concurrent Skip List

- **Objective**: Extend the skip list with lock-free concurrency.  
- **Implementation**:  
  - Use atomics to manage node pointers.
  - Design the data structure to support concurrent access.

---

### 9. Wait-free Queue Implementation

- **Objective**: Implement a queue where operations complete in a bounded number of steps, regardless of other threads.  
- **Implementation**:  
  - Use circular buffers with atomic indices for efficient wait-free operations.

---

### 10. Copy-on-write B+ Tree

- **Objective**: Build a B+ tree with copy-on-write semantics to optimize read and write operations.  
- **Implementation**:  
  - Clone nodes only when modifications are needed.
  - Ensure efficient access and update paths.

---

### 11. Lock-free Hash Map

- **Objective**: Create a lock-free hash map for concurrent data access.  
- **Implementation**:  
  - Use atomic operations for bucket management.
  - Handle collisions with concurrent-friendly techniques like chaining or open addressing.

---

### 12. Persistent Red-Black Tree

- **Objective**: Design a persistent red-black tree for efficient versioned access to data.  
- **Implementation**:  
  - Use immutable nodes with shared references.
  - Implement balancing rules for red-black trees.

---

### 13. Concurrent Bitmap Index

- **Objective**: Implement a bitmap index for fast queries in concurrent environments.  
- **Implementation**:  
  - Use atomic operations to modify the bitmap.
  - Optimize for memory usage and query performance.

---

### 14. Cache-oblivious B-tree

- **Objective**: Design a B-tree optimized for cache performance, independent of cache size.  
- **Implementation**:  
  - Structure the tree to minimize cache misses during traversal.
  - Implement search, insertion, and deletion.

---

### 15. Write-optimized LSM Tree

- **Objective**: Create a log-structured merge (LSM) tree for efficient writes in databases.  
- **Implementation**:  
  - Use write-ahead logs and in-memory tables.
  - Implement compaction strategies for merging data into disk storage.

---

## Experience and Learnings

Day 2's challenges provided hands-on experience with advanced data structures and concurrency patterns in Rust. Here are the key takeaways:

1. **Memory Safety**: Rust's ownership model simplifies resource management but requires careful planning for shared data structures.  
2. **Concurrency**: Implementing lock-free and wait-free structures highlighted the advantages of atomic operations and Rust's robust concurrency primitives.  
3. **Persistence**: Designing persistent data structures like the red-black tree deepened our understanding of immutability and shared references.  
4. **Optimization**: Tasks like cache-oblivious B-trees and LSM trees reinforced the importance of understanding hardware-level optimizations.  
