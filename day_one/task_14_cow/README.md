# Task: Copy-on-Write Data Structure Implementation

## Objective

The goal of this task was to implement a **Copy-on-Write (COW)** data structure in Rust. A COW data structure delays cloning data until a write operation occurs, which can improve memory efficiency. The primary objectives included:

1. **Efficient shared data access:** Allow multiple readers without cloning.
2. **Delayed modification cloning:** Ensure data is only cloned when a write occurs, minimizing unnecessary duplication.
3. **Concurrency support:** Safely handle concurrent access and modifications.

---

## Implementation Overview

The implementation was achieved using:

- **Arc** for reference-counted pointers to share ownership across threads.
- **RwLock** to allow concurrent readers but ensure exclusivity for writers.
- **Arc::make_mut** to clone the data only when shared ownership exists, maintaining the COW principle.

The core implementation is encapsulated in a `CopyOnWrite<T>` struct, where `T` represents the type of data being managed.

---

## Challenges and Encounters

### 1. **Understanding Arc::make_mut**

Initially, the use of `Arc::make_mut` caused type mismatch errors. Understanding that it only clones the inner data if it's shared took some time, but it was a critical insight for implementing the write method efficiently.

### 2. **Concurrency Edge Cases**

Simulating lock poisoning for testing was tricky. Rust’s type system and safety guarantees make it challenging to simulate unsafe scenarios like a poisoned lock. We resolved this by manually triggering a panic inside a `RwLock`.

### 3. **Compiler Lint: let_underscore_lock**

The compiler flagged a `non-binding let` error when acquiring a lock without using the returned guard. We addressed this by explicitly dropping the lock using `drop()` to align with Rust’s safety standards.

### 4. **Thread-Safety Requirements**

The combination of `RwLock` and `Arc` worked well for thread safety. However, ensuring that all operations maintained the integrity of the lock required careful handling, especially in tests with simulated failures.

---

## Key Learnings

1. **Rust’s Safety Guarantees**
   Working with `RwLock` and `Arc` highlighted how Rust’s ownership model and concurrency primitives provide strong guarantees against common bugs, like data races.

2. **Testing with Concurrency**
   Writing meaningful tests for concurrent data structures is challenging but rewarding. Simulating edge cases like lock poisoning provided a deeper understanding of `RwLock`.

3. **Arc::make_mut**
   This method is a powerful tool for COW patterns, but it requires careful attention to ownership semantics to avoid unnecessary cloning.

4. **The Compiler is Your Friend**
   Compiler errors, though frustrating at times, guided us toward better design choices. The `let_underscore_lock` warning, for example, helped ensure the lock was properly handled.

---

## Tests

We wrote comprehensive tests to validate the functionality:

1. **Happy Path:**
   - Read operations retrieve the correct data.
   - Write operations correctly clone and modify data.

2. **Unhappy Path:**
   - Simulated lock poisoning scenarios to verify the robustness of the lock recovery mechanism.

3. **Edge Cases:**
   - Handling empty data.
   - Stress-testing with large datasets.
