# Task: Reverse a Vector In-Place in Rust

## Task Description

You're tasked to implement an algorithm that reverses the elements of a vector in-place. The goal is:

Reverse: Rearrange the vector so that the last element becomes the first, the second-to-last becomes the second, and so on.

The implementation should:

Use a two-pointer approach, where one pointer starts at the beginning and another at the end, moving towards each other and swapping elements.
Be generic to work with any type T.

## Task Overview

This project implements a function to reverse the elements of a vector **in-place** using a two-pointer technique. The goal is to achieve this without using additional space, maintaining the original vector's memory location.

---

## Implementation

The `reverse_vector` function takes a mutable reference to a vector and reverses its elements by swapping them from both ends until the pointers meet.

### Code

```rust
fn reverse_vector<T>(vec: &mut Vec<T>) {
    let mut left = 0;
    let mut right = vec.len().checked_sub(1).unwrap_or(0);

    while left < right {
        vec.swap(left, right);
        left = left.checked_add(1).unwrap_or(usize::MAX);
        right = right.checked_sub(1).unwrap_or(0);
    }
}
```

## Considerations

### 1. **Addressing the Failure**

Overflow Error:
Initially, an overflow error was encountered when trying to access vec.len() - 1 for an empty vector. This happens because subtracting from zero in usize results in underflow. The issue occurred in the test case for an empty vector where right was set to -1 in an unsigned integer context, causing a panic.

### 2. **Using checked_sub**

Preventing Underflow:
usize::checked_sub was implemented to safely handle cases where len() is 0. If subtracting 1 from len() would result in underflow, checked_sub returns None. We use unwrap_or(0) to default right to 0 for empty vectors, ensuring no operations are performed when there are no elements to reverse.

### 3. **Implementation of checked_add**

Safety in Incrementing:
While left += 1 is inherently safe for this use case (since left will never exceed the vector's length in a valid scenario), checked_add was used for consistency and to adhere to Rust's safety principles. If an overflow were possible, checked_add would return None, and we’d default to usize::MAX to halt the loop (though this scenario doesn’t apply here).

## Testing

### Unit Tests

The implementation includes tests to verify the function's behavior.

**Happy Cases:**
Reversing even-length vectors.
Reversing odd-length vectors.
Verifying no change for single-element vectors.
Ensuring empty vectors remain unchanged.

**Unhappy Cases:**
Handling different types like characters to check generic functionality.
Testing with large vectors to ensure performance.

## How to Run

Test: Use `cargo test` to run all tests.
Run: Use `cargo run` to see an example of reversing a vector.

## Contributions

Feel free to contribute by suggesting safer methods, performance improvements, or additional test cases. Remember, Rust's philosophy is about safety and performance, so any optimization or safety enhancement is welcome.

## Conclusion

This project showcases basic operations with Rust vectors, emphasizing safety with checked_sub and checked_add for arithmetic operations on indices, ensuring the code can handle edge cases gracefully.

This README should provide clear context about the issues encountered, the solutions implemented, and how the code adheres to Rust's safety standards.
