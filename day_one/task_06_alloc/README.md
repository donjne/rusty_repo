# Task: Custom Memory Allocator using `std::alloc`

## Task Description

Implement a custom memory allocator in Rust using the GlobalAlloc trait. The allocator should:

Track memory allocations and deallocations.
Safely allocate and deallocate memory using std::alloc.
Provide a method to query the current amount of allocated memory.

## Task Overview

A memory allocator is a critical system component that manages dynamic memory. This project builds a custom allocator (CustomAllocator) that overrides the default memory allocator. The implementation:

**Tracks Memory Usage:**

Uses AtomicUsize for thread-safe tracking of allocated bytes.
Supports querying current memory usage through a now_allocated method.

**Implements GlobalAlloc:**

Overrides alloc to allocate memory and update the allocated count.
Overrides dealloc to free memory and decrement the allocated count.

**Includes Tests:**

Validates functionality through scenarios such as successful allocation, deallocation, handling zero-sized allocations, and attempting allocations with invalid layouts.

## Problem Analysis

### The Issue

While implementing and testing our custom memory allocator in Rust, we encountered a critical issue with error handling in our tests. Specifically, the test_unhappy_path failed because we used unwrap() on the result of Layout::from_size_align, which panics when an error occurs. The failure occurred in cases where the requested memory layout was invalid (e.g., an exceedingly large size).

### Root Cause

The issue arose because of an over-reliance on unwrapping results:

Layout Creation: The Layout::from_size_align function returns a Result that must be explicitly handled.
Invalid Input: When invalid input (e.g., an oversized layout) is passed, the function returns Err(LayoutError), which unwrap() cannot handle.
By unwrapping the result directly, the test terminated prematurely instead of properly handling the error as part of the test logic.

### The Solution

To address this, we refactored the error handling logic:

Match Statement: Replaced the unwrap() call with a match statement to explicitly handle both success (Ok) and failure (Err) cases.
On Ok, the layout proceeds with memory allocation.
On Err, the error is logged, and the test continues, avoiding unexpected panics.
Expected Failure Handling: For scenarios where Err is expected (e.g., oversized layouts), the test asserts that failure is correctly handled.

### Things to Note

Proper Error Handling: Always handle Result types explicitly, especially in test cases, to prevent unintentional panics and ensure comprehensive test coverage.

Testing Edge Cases: Simulating failure scenarios is as important as testing successful cases, particularly in low-level memory operations.
Readable Tests: Using match statements or other clear error-handling mechanisms improves the readability and robustness of the test code.

## Considerations

1. Safety: Rust enforces strict memory safety rules. Unsafe code is used carefully for low-level memory operations.

2. Concurrency: Thread safety is ensured using atomic operations for allocation tracking.

3. Compatibility: The allocator supports various memory layouts, provided they meet the GlobalAlloc constraints.

4. Error Handling: Properly handle errors from layout creation (Layout::from_size_align) to avoid panics.

## Other Pitfalls/Difficulties

**Managing Unsafe Code:**

Direct use of std::alloc functions requires careful handling to avoid undefined behavior.
Deallocating memory not allocated by this allocator could lead to undefined behavior.
Index Validation:

Ensuring valid memory alignment and size during layout creation is challenging and prone to runtime errors.

**Concurrency:**

Tracking memory allocation with AtomicUsize prevents race conditions but requires understanding atomic operations.

**Testing Edge Cases:**

Zero-sized allocations.
Large memory requests that exceed system limits.
Deallocating null or invalid pointers.

**Error Handling:**

Properly propagating and handling errors from Layout::from_size_align ensures robust testing and operation.

## Key Features of Implementation

Thread-safe Tracking: Uses atomic operations for concurrent environments.
Zero-cost Abstractions: Leverages Rust’s efficient standard library components.
Granular Tests: Includes test cases for edge cases, error handling, and performance scenarios.

## Lessons Learned

Understanding Rust's memory model and how the GlobalAlloc trait integrates with the allocator system is invaluable for systems programming.
Testing memory management requires thorough consideration of edge cases and potential undefined behavior.
Handling unsafe Rust code demands precision and an understanding of low-level system behavior.

## How to Run

Clone the repository.

Build and run using:

```bash
cargo run
```

Run tests:

```bash
cargo test
```
