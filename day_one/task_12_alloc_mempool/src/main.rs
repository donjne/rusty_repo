use std::collections::HashMap;

/// A structure representing a block of memory in the pool.
#[derive(Debug)]
struct MemoryBlock {
    size: usize,
    data: Vec<u8>,
}

/// The memory pool, which manages multiple blocks of memory.
#[derive(Debug)]
struct MemoryPool {
    pool: HashMap<usize, Vec<MemoryBlock>>, // Keyed by block size.
}

impl MemoryPool {
    /// Create a new memory pool.
    fn new() -> Self {
        MemoryPool {
            pool: HashMap::new(),
        }
    }

    /// Allocate a block of memory from the pool.
    fn allocate(&mut self, size: usize) -> Option<MemoryBlock> {
        let block = self.pool.entry(size).or_insert_with(Vec::new);
        
        // If there are no free blocks of this size, create a new one.
        if block.is_empty() {
            block.push(MemoryBlock {
                size,
                data: vec![0; size],
            });
        }

        block.pop()
    }

    /// Deallocate a block of memory and return it to the pool.
    fn deallocate(&mut self, block: MemoryBlock) {
        let block_size = block.size;
        let entry = self.pool.entry(block_size).or_insert_with(Vec::new);
        entry.push(block);
    }

    /// Allocate fixed-size blocks.
    fn allocate_fixed_size(&mut self, size: usize) -> Option<MemoryBlock> {
        self.allocate(size)
    }

    /// Allocate variable-size blocks.
    fn allocate_variable_size(&mut self, min_size: usize, max_size: usize) -> Option<MemoryBlock> {
        // Find the smallest block that fits within the specified range
        for size in min_size..=max_size {
            if let Some(blocks) = self.pool.get_mut(&size) {
                if let Some(block) = blocks.pop() {
                    return Some(block); // Return only if an existing block is found
                }
            }
        }
        None // Return None if no block is found in the range
    }

    /// Deallocate a block of memory.
    fn deallocate_block(&mut self, block: MemoryBlock) {
        self.deallocate(block);
    }
}

fn main() {
    let mut pool = MemoryPool::new();

    // Test fixed-size allocation
    if let Some(block) = pool.allocate_fixed_size(1024) {
        println!("Allocated fixed-size block of size {}: {:?}", block.size, block.data);
    }

    // Test variable-size allocation
    if let Some(block) = pool.allocate_variable_size(512, 2048) {
        println!("Allocated variable-size block of size {}: {:?}", block.size, block.data);
    }

    // Deallocate a block
    let block_to_deallocate = MemoryBlock {
        size: 1024,
        data: vec![0; 1024],
    };
    pool.deallocate_block(block_to_deallocate);

    // Show memory pool state after deallocation
    println!("Memory pool after deallocation: {:?}", pool);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_fixed_size() {
        let mut pool = MemoryPool::new();
        
        // Allocate a fixed-size block
        let block = pool.allocate_fixed_size(1024).expect("Allocation failed");
        assert_eq!(block.size, 1024);
        assert_eq!(block.data.len(), 1024);
    }

    #[test]
    fn test_allocate_variable_size() {
        let mut pool = MemoryPool::new();
    
        // Prepopulate the pool with blocks of various sizes
        pool.deallocate_block(MemoryBlock {
            size: 512,
            data: vec![0; 512],
        });
        pool.deallocate_block(MemoryBlock {
            size: 1024,
            data: vec![0; 1024],
        });
        pool.deallocate_block(MemoryBlock {
            size: 2048,
            data: vec![0; 2048],
        });
    
        // Allocate a variable-size block between 512 and 2048 bytes
        let block = pool
            .allocate_variable_size(512, 2048)
            .expect("Allocation failed");
        assert!(block.size >= 512 && block.size <= 2048);
        assert_eq!(block.data.len(), block.size);
    }
    

    #[test]
    fn test_deallocate_block() {
        let mut pool = MemoryPool::new();
        
        // Allocate and deallocate a block
        let block = pool.allocate_fixed_size(1024).expect("Allocation failed");
        pool.deallocate_block(block);
        
        // Verify that the pool has the deallocated block
        let deallocated_block = pool.allocate_fixed_size(1024).expect("Allocation failed");
        assert_eq!(deallocated_block.size, 1024);
    }

    #[test]
    fn test_allocate_variable_size_no_blocks() {
        let mut pool = MemoryPool::new();
        
        // Try to allocate a block with a size range that doesn't exist in the pool
        let block = pool.allocate_variable_size(5000, 10000);
        assert!(block.is_none(), "Expected None, but got a block");
    }

    #[test]
    fn test_deallocate_empty_block() {
        let mut pool = MemoryPool::new();
        
        // Deallocate an empty block (which should not exist)
        let block = MemoryBlock {
            size: 0,
            data: Vec::new(),
        };
        pool.deallocate_block(block); // Should not panic
    }

    #[test]
    fn test_allocate_minimum_block_size() {
        let mut pool = MemoryPool::new();
        
        // Allocate the smallest possible block (e.g., 1 byte)
        let block = pool.allocate_fixed_size(1).expect("Allocation failed");
        assert_eq!(block.size, 1);
    }

    #[test]
    fn test_allocate_maximum_block_size() {
        let mut pool = MemoryPool::new();
        
        // Allocate a large block, assuming the system can handle large allocations
        let block = pool.allocate_fixed_size(1000000).expect("Allocation failed");
        assert_eq!(block.size, 1000000);
    }
}
