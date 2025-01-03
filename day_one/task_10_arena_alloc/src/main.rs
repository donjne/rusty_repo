struct MemoryArena {
    memory: Vec<u8>,  // This will hold the pre-allocated memory block.
    current: usize,   // The current position to allocate from.
}

impl MemoryArena {
    // Create a new arena with a given size
    pub fn new(size: usize) -> Self {
        let mut memory = Vec::with_capacity(size);
        unsafe {
            // Fill the allocated memory with zeroes (simulate pre-allocation)
            memory.set_len(size);
        }
        MemoryArena {
            memory,
            current: 0,  // Start at the beginning of the arena.
        }
    }

    // Allocate a chunk of memory from the arena
    pub fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        // If size is 0, allocation should fail
        if size == 0 {
            return None;
        }

        // Ensure there is enough space in the arena
        if self.current + size <= self.memory.len() {
            let ptr = self.memory[self.current..].as_mut_ptr();
            self.current += size;
            Some(ptr)
        } else {
            // Not enough space
            None
        }
    }

    // Reset the arena (optional, for reusing the memory block)
    pub fn reset(&mut self) {
        self.current = 0; // Reset the allocation pointer to the start
    }

    // Return the remaining available memory in the arena
    pub fn remaining(&self) -> usize {
        self.memory.len() - self.current // Calculate how much memory is left
    }
}

impl Drop for MemoryArena {
    fn drop(&mut self) {
        // Memory will be freed when the arena is dropped
        println!("Arena is being dropped and memory is deallocated.");
    }
}

fn main() {
    // Create an arena with 1024 bytes
    let mut arena = MemoryArena::new(1024);

    // Allocate 100 bytes from the arena
    if let Some(ptr) = arena.allocate(100) {
        println!("Allocated 100 bytes.");

        // Use the pointer to store values directly into the allocated memory
        unsafe {
            // Example: Fill the allocated memory with a simple pattern (e.g., numbers 0 to 99)
            for i in 0..100 {
                *ptr.add(i) = i as u8; // Store values 0 to 99 in the allocated memory
            }
        }

        // Read back the values from the allocated memory and print them
        unsafe {
            let values: Vec<u8> = (0..100)
                .map(|i| *ptr.add(i))  // Read each byte from the allocated memory
                .collect();
            println!("First 10 values allocated: {:?}", &values[0..10]);
        }
    } else {
        println!("Failed to allocate 100 bytes.");
    }

    // Allocate another chunk (200 bytes)
    if let Some(ptr) = arena.allocate(200) {
        println!("Allocated 200 bytes.");

        // Similarly, fill the next 200 bytes with a different pattern (e.g., values 100 to 299)
        unsafe {
            for i in 0..200 {
                *ptr.add(i) = (i + 100) as u8; // Store values 100 to 299 in the allocated memory
            }
        }

        // Read back the values from the allocated memory
        unsafe {
            let values: Vec<u8> = (0..200)
                .map(|i| *ptr.add(i))  // Read each byte from the allocated memory
                .collect();
            println!("First 10 values allocated in second chunk: {:?}", &values[0..10]);
        }
    } else {
        println!("Failed to allocate 200 bytes.");
    }

    // Check the remaining memory
    println!("Remaining memory: {} bytes", arena.remaining());

    // Reset the arena (reuse the memory block)
    arena.reset();
    println!("Arena has been reset.");
    println!("Remaining memory after reset: {} bytes", arena.remaining());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_successfully() {
        let mut arena = MemoryArena::new(1024); // Create arena with 1024 bytes
        let chunk = arena.allocate(512); // Allocate 512 bytes
        assert!(chunk.is_some(), "Allocation should be successful");
        assert_eq!(arena.remaining(), 512, "Arena should have 512 bytes remaining");
    }

    #[test]
    fn test_allocate_multiple_chunks() {
        let mut arena = MemoryArena::new(1024); // Create arena with 1024 bytes
        let chunk1 = arena.allocate(256); // Allocate 256 bytes
        let chunk2 = arena.allocate(256); // Allocate another 256 bytes
        let chunk3 = arena.allocate(256); // Allocate another 256 bytes

        assert!(chunk1.is_some(), "First allocation should be successful");
        assert!(chunk2.is_some(), "Second allocation should be successful");
        assert!(chunk3.is_some(), "Third allocation should be successful");

        // Check remaining memory
        assert_eq!(arena.remaining(), 256, "Arena should have 256 bytes remaining");
    }

    #[test]
    fn test_allocate_more_than_available_space() {
        let mut arena = MemoryArena::new(1024); // Create arena with 1024 bytes
        let chunk = arena.allocate(1100); // Try to allocate 1100 bytes (more than available)

        assert!(chunk.is_none(), "Allocation should fail if there is not enough memory");
    }

    #[test]
    fn test_allocate_zero_size() {
        let mut arena = MemoryArena::new(1024); // Create arena with 1024 bytes
        let chunk = arena.allocate(0); // Try to allocate 0 bytes

        assert!(chunk.is_none(), "Allocation of 0 bytes should fail");
    }

    #[test]
    fn test_allocate_large_chunk() {
        let mut arena = MemoryArena::new(1024); // Create arena with 1024 bytes
        let chunk = arena.allocate(1025); // Try to allocate 1025 bytes (larger than arena)

        assert!(chunk.is_none(), "Allocation should fail if the requested size is larger than the arena");
    }

    #[test]
    fn test_reset_arena() {
        let mut arena = MemoryArena::new(1024); // Create arena with 1024 bytes
        let chunk1 = arena.allocate(512); // Allocate 512 bytes
        let chunk2 = arena.allocate(256); // Allocate 256 bytes
        assert!(chunk1.is_some(), "First allocation should be successful");
        assert!(chunk2.is_some(), "Second allocation should be successful");

        // Reset the arena and check remaining memory
        arena.reset();
        assert_eq!(arena.remaining(), 1024, "Arena should be reset to full capacity");
    }
}
