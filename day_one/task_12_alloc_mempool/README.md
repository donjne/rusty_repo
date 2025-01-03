# Memory Pool Manager in Rust

## Overview

This project implements a **Memory Pool Manager** in Rust. It is designed to manage blocks of memory efficiently, enabling fixed-size and variable-size block allocations while providing mechanisms to recycle deallocated blocks. This project was an excellent learning experience, offering deep insights into memory management, Rust's ownership model, and the importance of robust testing.

## Features

- **Fixed-size block allocation:** Allocate memory blocks of a specific size.
- **Variable-size block allocation:** Dynamically allocate memory blocks within a specified size range.
- **Deallocation and recycling:** Return memory blocks to the pool for future use, reducing resource waste.
- **Efficient storage:** Utilize `HashMap` to group memory blocks by size for quick access.

## Project Experience

### Challenges

1. **Handling Variable-size Allocation:**
   - Initial implementation failed to allocate blocks when no pre-existing blocks were available in the pool. This revealed the importance of prepopulating the memory pool or implementing logic to create blocks dynamically.

2. **Managing Ownership:**
   - Rust’s ownership model posed challenges when handling mutable references to memory blocks. This encouraged us to explore idiomatic patterns for managing shared resources efficiently.

3. **Testing for Edge Cases:**
   - Writing robust test cases exposed potential issues such as:
     - Allocating from an empty pool.
     - Deallocating blocks of unusual sizes (e.g., 0 bytes).
     - Handling large memory blocks that could potentially exhaust the system's resources.

### Key Learnings

1. **Prepopulating the Memory Pool:**
   - Preloading the pool with memory blocks ensures smooth operations, especially for variable-size allocation scenarios.

2. **Test-Driven Development (TDD):**
   - Writing tests first helped us identify edge cases early, leading to a more resilient implementation.

3. **Error Handling in Rust:**
   - Rust’s `Option` and `Result` types were invaluable for managing cases where allocation or deallocation might fail.

4. **HashMap for Efficient Grouping:**
   - Using a `HashMap` to group blocks by size significantly improved allocation and deallocation performance.

### Improvements for Future Iterations

1. **Dynamic Block Creation:**
   - Extend the variable-size allocation method to create blocks dynamically if none are found in the specified range.

2. **Metrics Collection:**
   - Add logging or metrics to monitor pool usage and identify potential bottlenecks.

3. **Memory Safety Guarantees:**
   - Explore integrating Rust’s `unsafe` features judiciously for more granular control over memory while maintaining safety.

4. **Concurrency Support:**
   - Introduce thread-safe mechanisms, such as `Mutex` or `RwLock`, to enable multi-threaded access to the memory pool.

## Code Example

### Fixed-size Allocation

```rust
if let Some(block) = pool.allocate_fixed_size(1024) {
    println!("Allocated fixed-size block of size {} bytes", block.size);
}
```

### Variable-size Allocation

```rust
if let Some(block) = pool.allocate_variable_size(512, 2048) {
    println!("Allocated variable-size block of size {} bytes", block.size);
}
```

### Deallocation

```rust
let block_to_deallocate = MemoryBlock {
    size: 1024,
    data: vec![0; 1024],
};
pool.deallocate_block(block_to_deallocate);
```

## How to Run

1. Clone this repository:

   ```bash
   git clone <repository-url>
   cd memory-pool-manager
   ```

2. Build and run the project:

   ```bash
   cargo run
   ```

3. Run the tests:

   ```bash
   cargo test
   ```

## Conclusion

This project was an enlightening dive into efficient memory management using Rust. It honed our skills in designing resource-efficient systems and reinforced the importance of robust testing in software development. The lessons learned here will undoubtedly be invaluable in future endeavors.
