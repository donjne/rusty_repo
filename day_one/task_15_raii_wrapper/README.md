# Task: RAII Wrapper for System Resources

## Overview

This project implements a RAII (Resource Acquisition Is Initialization) wrapper in Rust to manage system resources like file handles automatically. The wrapper ensures that resources are acquired when the object is created and safely released when the object goes out of scope, leveraging Rust's ownership model and the `Drop` trait.

## Features

- Encapsulation of resource management using a custom struct (`FileWrapper`).
- Automatic release of system resources when the wrapper goes out of scope.
- Safe and ergonomic API for interacting with system resources.
- Comprehensive testing for happy, unhappy, and edge cases.

## Code Implementation

The main logic revolves around creating a struct `FileWrapper` that wraps a file resource. The `Drop` trait is implemented to handle resource cleanup automatically. A detailed implementation is available in the `main.rs` file, along with methods for interacting with the resource (e.g., writing data).

## Testing Strategy

We wrote tests to verify the functionality and robustness of our RAII wrapper. These include:

1. **Happy Path:**
   - Writing data to a file and verifying the content.
   - Confirming that the resource is released when the wrapper goes out of scope.

2. **Unhappy Path:**
   - Handling scenarios where the file resource is unavailable.
   - Attempting invalid operations and verifying the error handling.

3. **Edge Cases:**
   - Writing empty content to a file.
   - Writing large content to ensure performance and stability.

4. **Clean-Up:**
   - Ensuring test files are removed after each test to maintain a clean environment.

## Challenges Encountered

- **Understanding `Drop` Behavior:**
  Initially, we struggled to understand when and how the `Drop` trait is called. Through documentation and experimentation, we learned how Rust ensures deterministic cleanup.

- **Error Handling:**
  Managing error scenarios such as file permission issues and handling `None` values for the resource required careful planning to avoid panics.

- **Resource Safety:**
  Ensuring the resource was properly released without leaks or misuse was a priority. We spent considerable time validating this through various edge cases.

## Lessons Learned

- **RAII in Rust:**
  This task deepened our understanding of how Rust leverages RAII to enforce resource safety. The combination of ownership, borrowing, and the `Drop` trait makes it intuitive to manage resources.

- **Testing Discipline:**
  Writing tests for both common and rare scenarios is essential for robust implementations. Edge cases often uncover subtle bugs.

- **Iterative Debugging:**
  Encountering runtime errors during development helped us refine our approach to error handling and resource management.

## How to Run

1. Clone this repository.
2. Navigate to the project directory.
3. Run the main application:

   ```bash
   cargo run --bin task_14_cow
   ```

4. Execute tests to verify functionality:

   ```bash
   cargo test
   ```

## Example Output

When running the main function, you should see:

```plaintext
Data written to the file successfully.
File resource released.
```
