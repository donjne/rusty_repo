# Day 1: Implementing Data Structures and Memory Management in Rust

## Objective

The focus of Day 1 is to lay the groundwork in Rust by implementing fundamental data structures and exploring advanced memory management techniques. This day's tasks combine basic data structure implementations with deeper dives into Rust's unique memory safety features.

## Tasks Overview

1. **Implement a Stack using `Vec`**
2. **Create a Queue using two Stacks**
3. **Reverse a Vector In-place**
4. **Implement a Circular Buffer**
5. **Create a Fixed-size Ring Buffer**
6. **Implement Custom Allocator using `std::alloc`**
7. **Write a Simple Memory Pool**
8. **Implement Custom Smart Pointer with Interior Mutability**
9. **Create Lock-free Data Structure using Atomic Operations**
10. **Build Memory Arena Allocator**
11. **Implement Reference-counted Garbage Collector**
12. **Create Custom Allocator with Memory Pooling**
13. **Design Zero-copy Buffer Management System**
14. **Implement Copy-on-write Data Structure**
15. **Create RAII Wrapper for System Resources**

---

## Detailed Task Descriptions

### 1. Implement a Stack using `Vec`

A stack is a data structure that follows the Last In, First Out (LIFO) principle. We implemented it using `Vec<T>`, allowing push and pop operations to add and remove items.

- **Objective**: Develop a stack structure using `Vec<T>` where elements are added and removed from the top (LIFO - Last In, First Out).
- **Implementation**:
  - Use `Vec<T>` for storage.
  - Implement `push` to add an element to the top of the stack.
  - Implement `pop` to remove and return the top element, or handle if the stack is empty.

### 2. Create a Queue using Two Stacks

A queue follows the First In, First Out (FIFO) principle. We can implement a queue using two stacks: one for enqueuing and one for dequeuing.

- **Objective**: Implement a queue using two stacks to ensure FIFO (First In, First Out) behavior.
- **Implementation**:
  - Use one stack for enqueuing (inserting) elements.
  - Use another stack for dequeuing (removing) elements by reversing the order from the first stack when necessary.

### 3. Reverse a Vector In-place

To reverse a vector in-place, we swap elements starting from the two ends of the vector, moving towards the center.

- **Objective**: Reverse the order of elements in a vector without using extra space.
- **Implementation**:
  - Use a two-pointer approach, swapping elements from both ends towards the middle until the pointers meet.

### 4. Implement a Circular Buffer

A circular buffer (or ring buffer) is a fixed-size buffer that wraps around. When the buffer is full, new elements overwrite the oldest ones.

- **Objective**: Create a buffer where elements are stored in a circular manner, allowing for efficient reuse of space.
- **Implementation**:
  - Define `push`, `pop`, and `is_empty` methods.
  - Handle overflow by overwriting oldest data when full.

### 5. Create a Fixed-size Ring Buffer

A ring buffer is a circular buffer with a fixed size. Once the buffer is full, new elements replace the oldest elements in a circular fashion.

- **Objective**: Similar to the circular buffer but with a fixed size where older elements are automatically overwritten with new ones.
- **Implementation**:
  - Implement with a fixed size array or `Vec` with a known capacity.
  - Manage read and write indices to ensure circular access.

### 6. Implement Custom Allocator using `std::alloc`

A custom allocator is used to manage memory allocation and deallocation for more control over memory usage.

- **Objective**: Write a custom allocator to control memory allocation and deallocation.
- **Implementation**:
  - Use `std::alloc` to manage raw memory.
  - Ensure safe memory handling with Rust's ownership model.

### 7. Write a Simple Memory Pool

A memory pool is a custom allocator that pre-allocates memory to avoid frequent allocations and deallocations.

- **Objective**: Pre-allocate memory chunks to reduce allocation overhead for frequently created and destroyed objects.
- **Implementation**:
  - Pre-allocate a block of memory.
  - Implement methods to allocate from and return to the pool.

### 8. Implement Custom Smart Pointer with Interior Mutability

A smart pointer manages memory automatically. Implement a smart pointer that uses interior mutability to allow mutable access, adhering to Rust's borrowing rules, and ensuring safe usage.

- **Objective**: Create a smart pointer that uses RefCell for interior mutability, enabling mutable access even through an immutable reference.
- **Implementation**:
  - Use `RefCell` to manage the inner value and enforce borrowing rules at runtime.
  - Provide methods for immutable and mutable access (`borrow` and `borrow_mut`) that showcase safe usage
  - Demonstrate the smart pointer's functionality with examples in the main function, ensuring no runtime borrow violations occur.

### 9. Create Lock-free Data Structure using Atomic Operations

A lock-free data structure allows multiple threads to access the data concurrently without locking.

- **Objective**: Design a data structure that supports concurrent access without traditional locking mechanisms.
- **Implementation**:
  - Use atomic types (`AtomicUsize`, etc.) for thread-safe operations.
  - Consider a lock-free stack or queue for this task.

### 10. Build Memory Arena Allocator

A memory arena allocator pre-allocates a large block of memory and manages it in chunks.

- **Objective**: Allocate large blocks of memory upfront and manage them efficiently for smaller allocations.
- **Implementation**:
  - Create an arena that allocates from a single large chunk.
  - Implement methods to allocate within this arena and manage memory.

### 11. Implement Reference-counted Garbage Collector

A garbage collector automatically reclaims memory that is no longer in use, using reference counting.

- **Objective**: Use reference counting for automatic memory management.
- **Implementation**:
  - Utilize `Rc` or `Arc` for managing shared ownership.
  - Implement a garbage collection method for cleaning up when reference counts drop to zero.

### 12. Create Custom Allocator with Memory Pooling

This task involves creating an allocator that uses a memory pool for memory management. The implementation is similar to the earlier memory pool but with more customization for memory management.

- **Objective**: Enhance the previous memory pool with custom allocation strategies.
- **Implementation**:
  - Allow for different sizes or types of allocations within the pool.
  - Optimize for specific use cases by varying block sizes.

### 13. Design Zero-copy Buffer Management System

Zero-copy buffers allow data to be passed between memory locations without copying it, making the system more efficient.

- **Objective**: Implement a system where data can be shared between buffers without copying.
- **Implementation**:
  - Use shared memory or memory mapping techniques.
  - Design a structure where multiple consumers can read from a single buffer.

### 14. Implement Copy-on-write Data Structure

A copy-on-write (COW) data structure allows modifications to be delayed until necessary, making it memory efficient.

- **Objective**: Create a data structure where modifications only occur when a write happens, potentially saving memory.
- **Implementation**:
  - Use `Rc` or `Arc` with `RefCell` or `Mutex` for shared mutable state.
  - Implement logic to clone on write.

### 15. Create RAII Wrapper for System Resources

RAII (Resource Acquisition Is Initialization) is a programming idiom where resources are acquired when an object is created and released when it goes out of scope.

- **Objective**: Ensure system resources are automatically managed using Rust's RAII idiom.
- **Implementation**:
  - Wrap a resource (like file handles or network sockets) in a struct.
  - Use `Drop` trait to release the resource when the struct goes out of scope.

---

This day's exercises are designed to deepen your understanding of Rust's memory management and data structures, emphasizing efficiency, safety, and concurrency. Each task pushes you towards mastering these concepts in practical scenarios.
