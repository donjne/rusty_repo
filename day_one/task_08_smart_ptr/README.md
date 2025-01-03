# Custom Smart Pointer with Interior Mutability

## Overview

This project demonstrates the creation of a custom smart pointer in Rust that utilizes interior mutability, allowing mutable access to its value even when the smart pointer itself is immutable. This functionality is achieved through the use of `RefCell`.

## Features Implemented

1. **Interior Mutability**:
   - Enabled through `RefCell`, allowing runtime borrow checks for safe mutable access.
   - Supports controlled, mutable access while adhering to Rust's ownership and borrowing rules.

2. **Basic Ownership Semantics**:
   - Ensures that the smart pointer adheres to Rust's ownership model while providing interior mutability.

3. **Trait Implementations**:
   - Implemented the `Deref` and `DerefMut` traits for seamless usage of the smart pointer like regular references.

4. **Safe Borrowing**:
   - Demonstrates proper usage of `borrow` and `borrow_mut` methods to avoid runtime panics.

## Here Are Some Features Not Yet Implemented

1. **Advanced Error Handling**:
   - Borrowing violations will still result in runtime panics rather than custom error handling or recovery mechanisms.

2. **Thread Safety**:
   - The smart pointer is not thread-safe. Features like `Mutex` or `RwLock` were not incorporated for concurrent environments.

3. **Shared Ownership**:
   - The implementation does not include reference counting (`Rc` or `Arc`) for shared ownership scenarios.

4. **Advanced Ergonomics**:
   - No additional convenience methods were added beyond basic borrowing and dereferencing functionalities.

## Usage

### Initialization

```rust
let smart_pointer = CustomSmartPointer::new(42); // Wraps a value
```

### Immutable Access

```rust
println!("Value: {}", *smart_pointer); // Uses Deref for immutable access
```

### Mutable Access

```rust
*smart_pointer.borrow_mut() = 100; // Updates value with borrow_mut
println!("Updated Value: {}", *smart_pointer);
```

## Pitfalls and Challenges

- **Runtime Panics**: Misusing `borrow` or `borrow_mut` can lead to runtime panics if borrowing rules are violated.
- **Ownership and Mutability**: The implementation assumes careful usage to avoid issues with mutable and immutable borrows.

## Lessons Learned

- `RefCell` provides flexibility but requires strict runtime adherence to borrowing rules.
- Implementing `Deref` and `DerefMut` enhances ergonomics, making the smart pointer behave like regular references.
- Designing safe, flexible smart pointers involves balancing Rust’s ownership model with runtime checks.

## Testing

To verify the implementation:

```bash
cargo test
```

Tests include:

- Validation of `Deref` and `DerefMut` behavior.
- Borrowing rule enforcement with `RefCell`.
- Handling of edge cases, such as nested mutable borrows.

## Future Improvements

1. **Thread Safety**:
   - Incorporate `Mutex` or `RwLock` for interior mutability in multithreaded environments.

2. **Shared Ownership**:
   - Add support for shared ownership using `Rc` or `Arc`.

3. **Enhanced Error Handling**:
   - Implement custom error messages or recovery mechanisms for borrowing violations.

4. **Extended Features**:
   - Provide additional utility methods for more ergonomic usage.
