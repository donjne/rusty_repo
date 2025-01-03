# Task: Implement Reference-counted Garbage Collector ~ Rust Memory Management with `Arc`, `Mutex`, `Rc`, and `RefCell`

## Introduction

In this project, we explored more of Rust's powerful memory management mechanisms, focusing on **reference counting** and **interior mutability** using `Arc`, `Mutex`, `Rc`, and `RefCell`. These tools enable safe and efficient management of memory and shared state, particularly in multi-threaded environments. The aim was to implement a **Reference-Counted Garbage Collector** (RCGC) using `Arc` and `Mutex` for thread safety, and explore how `Rc` and `RefCell` provide flexible ownership and mutation of data in single-threaded contexts.

## Key Concepts

### **Arc (Atomic Reference Counted)**

`Arc` is a thread-safe, atomic version of `Rc` (Reference Counted), enabling shared ownership of data across multiple threads. It maintains a reference count that is updated atomically, ensuring that when the reference count drops to zero, the data is deallocated. The primary advantage of `Arc` is its ability to allow multiple threads to safely share ownership of data.

- **Automatic Memory Cleanup**: Data is deallocated when the reference count reaches zero, eliminating the need for manual memory management.
- **Thread Safety**: `Arc` is designed for concurrent environments, ensuring that multiple threads can safely clone and share references to the same data.

### **Mutex (Mutual Exclusion)**

To modify data shared across threads, we pair `Arc` with `Mutex`. A `Mutex` ensures that only one thread can access the data at any time, preventing race conditions and guaranteeing safe mutability. The combination of `Arc` and `Mutex` enables us to mutate shared data across multiple threads without violating Rust's safety guarantees.

- **Thread-Safe Mutation**: Only one thread can mutate the data at a time, while other threads must wait for the lock to be released.

### **Rc (Reference Counting)**

`Rc` is a reference-counted pointer, which allows multiple ownership of data within a single thread. Unlike `Arc`, `Rc` is not thread-safe and is suited for single-threaded contexts. It ensures that memory is automatically freed when the last reference to the data is dropped.

- **Automatic Memory Cleanup**: Like `Arc`, `Rc` tracks the reference count, deallocating memory once no more references exist.
- **Shared Ownership**: Multiple references to the same data can coexist, enabling shared ownership in single-threaded environments.

### **RefCell (Interior Mutability)**

`RefCell` enables mutable access to data even when the data is behind an immutable reference. Unlike the typical Rust borrowing rules, `RefCell` enforces the borrowing rules at runtime rather than compile time. This allows us to mutate data behind a shared reference, providing flexibility in managing state.

- **Dynamic Borrowing**: `RefCell` dynamically checks if the data is being borrowed immutably or mutably and ensures that there are no violations at runtime.
- **Interior Mutability**: It allows mutable access to data even if the data is shared across multiple references.

## Implementation: **ReferenceCountedGC**

In this project, we implemented a custom **ReferenceCountedGC** structure, which combines both `Rc` and `RefCell` to manage shared and mutable data safely. Here's how we approached the problem:

1. **Creating `MyData`**: We defined a simple structure, `MyData`, containing an integer value. This value needed to be shared across different parts of the program and mutated at runtime. To achieve this, we encapsulated it in a `RefCell` to enable interior mutability.

2. **Reference Counting**: We wrapped the `RefCell` in an `Rc` to enable multiple references to `MyData` while automatically managing the memory via reference counting.

3. **Access and Modification**: We provided methods for accessing and modifying the data using `RefCell::borrow()` for immutable access and `RefCell::borrow_mut()` for mutable access, ensuring safe data manipulation.

### Challenges Encountered

- **Handling Thread Safety**: We needed to ensure thread safety when multiple threads were accessing or modifying shared data. By using `Arc` and `Mutex`, we ensured that references to data were safely shared across threads, and mutations were synchronized.
  
- **Understanding Ownership and Borrowing**: Rust's ownership and borrowing rules are strict, and we had to ensure that references were not inadvertently moved. Using `Arc` and `Mutex` with `RefCell` helped us manage ownership and mutability more effectively.

- **Runtime Borrowing**: Using `RefCell` came with the challenge of ensuring that mutable and immutable references were not mixed inappropriately. Rust's runtime borrowing checks helped us catch violations early.

### Solutions We Came Up With

- **Using `Arc` and `Mutex` for Shared Ownership in Multi-threaded Environments**: `Arc` allowed us to share ownership of the data across threads safely, and `Mutex` ensured that only one thread could mutate the data at any time.

- **Managing the Reference Count**: We manually tracked the reference count using `Arc` to simulate the behavior of a garbage collector. We observed that once the reference count dropped to zero, the memory was cleaned up automatically.

- **Testing and Validation**: We implemented a comprehensive suite of tests, validating reference counting, thread-safety, and proper memory cleanup. Tests also covered edge cases, such as handling the cleanup process when references are dropped in different orders.

## Multi-threaded Extensions with `Arc` and `Mutex`

In addition to the single-threaded examples using `Rc` and `RefCell`, we also explored the multi-threaded use of `Arc` and `Mutex`. The `Arc` type allows us to safely share data across multiple threads, and the `Mutex` provides the necessary locking to mutate that data safely.

- **Shared Ownership Across Threads**: Using `Arc`, we cloned references and passed them across threads. Each thread could safely access and modify the data.
- **Mutex for Safe Mutation**: `Mutex` ensured that only one thread could mutate the data at any given time, preventing data races and ensuring consistent state.

### Example Test for Multi-threaded Access

To demonstrate the correctness of the approach, we wrote a test that spawns multiple threads, each of which attempts to modify the shared data. The test verified that the data was safely mutated and that the reference count behaved as expected.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct SharedData {
    value: i32,
}

fn main() {
    let data = Arc::new(Mutex::new(SharedData { value: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            data.value += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", data.lock().unwrap().value);
}
```

In this example, multiple threads increment the `value` field, ensuring that only one thread can mutate the data at a time due to the `Mutex`.

## Conclusion

This project provided a deep dive into Rust's memory management features, focusing on **reference counting** with `Arc` and `Rc`, and **interior mutability** with `RefCell`. By combining these tools, we were able to implement a custom reference-counted garbage collector that works efficiently in both single-threaded and multi-threaded environments.

- **Automatic Memory Management**: Rust’s ownership model, combined with `Arc` and `Rc`, allows automatic cleanup of memory when references are dropped.
- **Thread-Safety**: Using `Mutex` with `Arc` ensures that data can be safely accessed and mutated across multiple threads without violating Rust's safety guarantees.
- **Flexibility and Safety**: `RefCell` provided the necessary flexibility for mutable access to data behind shared references, while still adhering to Rust's strict safety rules.

This experience has deepened our understanding of Rust's powerful memory model and how these tools work together to allow for safe, efficient, and flexible data management. We are now well-equipped to leverage these concepts in more advanced Rust projects, especially those that require managing shared state in multi-threaded or concurrent environments.
