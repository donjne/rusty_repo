use std::sync::{Arc, Mutex};

struct MemoryPool {
    pool: Mutex<Vec<Vec<u8>>>,
    chunk_size: usize,
    capacity: usize,
}

impl MemoryPool {
    /// Creates a new memory pool with a specified chunk size and number of chunks.
    pub fn new(chunk_size: usize, capacity: usize) -> Arc<Self> {
        let pool = (0..capacity)
            .map(|_| vec![0; chunk_size])
            .collect::<Vec<_>>();
        Arc::new(Self {
            pool: Mutex::new(pool),
            chunk_size,
            capacity,
        })
    }

    /// Allocates a chunk from the pool. Returns None if the pool is exhausted.
    pub fn allocate(&self) -> Option<Vec<u8>> {
        let mut pool = self.pool.lock().unwrap();
        pool.pop()
    }

    /// Returns a chunk back to the pool.
    pub fn deallocate(&self, chunk: Vec<u8>) {
        if chunk.len() == self.chunk_size {
            let mut pool = self.pool.lock().unwrap();
            if pool.len() < self.capacity {
                pool.push(chunk);
            }
        } else {
            panic!("Chunk size does not match the pool's chunk size.");
        }
    }

    /// Checks the number of available chunks in the pool.
    pub fn available_chunks(&self) -> usize {
        let pool = self.pool.lock().unwrap();
        pool.len()
    }
}

fn main() {
    let pool = MemoryPool::new(1024, 10);

    println!("Pool created with capacity for 10 chunks of 1024 bytes each.");

    // Allocate some chunks
    let chunk1 = pool.allocate().expect("First allocation should succeed");
    let chunk2 = pool.allocate().expect("Second allocation should succeed");

    println!("Allocated two chunks. Chunks available: {}", pool.available_chunks());

    // Use the chunks
    println!("Chunk1 size: {} bytes", chunk1.len());
    println!("Chunk2 size: {} bytes", chunk2.len());

    // Deallocate one chunk
    pool.deallocate(chunk1);
    println!("Deallocated one chunk. Chunks available: {}", pool.available_chunks());

    // Try to allocate again
    let chunk3 = pool.allocate().expect("Reallocation should succeed after deallocation");
    println!("Reallocated a chunk. Chunks available: {}", pool.available_chunks());

    // This will fail since we've used up all chunks
    if pool.allocate().is_none() {
        println!("Failed to allocate more chunks; pool is exhausted.");
    }

    // Deallocate remaining chunks
    pool.deallocate(chunk2);
    pool.deallocate(chunk3);
    println!("All chunks deallocated. Chunks available: {}", pool.available_chunks());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool() {
        let pool = MemoryPool::new(1024, 10);

        // Allocate all chunks
        let mut allocated = Vec::new();
        for _ in 0..10 {
            let chunk = pool.allocate().expect("Should allocate successfully");
            assert_eq!(chunk.len(), 1024);
            allocated.push(chunk);
        }

        // Pool should be exhausted
        assert!(pool.allocate().is_none(), "Pool should be exhausted");

        // Deallocate a chunk
        pool.deallocate(allocated.pop().unwrap());
        assert_eq!(pool.available_chunks(), 1, "One chunk should be available");

        // Reallocate the chunk
        let chunk = pool.allocate().expect("Should allocate successfully");
        assert_eq!(chunk.len(), 1024);
    }

    #[test]
    #[should_panic]
    fn test_invalid_deallocate() {
        let pool = MemoryPool::new(1024, 10);
        // Attempt to deallocate a chunk with an invalid size
        pool.deallocate(vec![0; 512]);
    }
}