use std::sync::{Arc, RwLock};

/// A Zero-Copy Buffer structure for managing data.
#[derive(Debug, Clone)]
struct ZeroCopyBuffer {
    data: Arc<RwLock<Vec<u8>>>,
}

impl ZeroCopyBuffer {
    /// Create a new buffer with the given data.
    fn new(data: Vec<u8>) -> Self {
        Self {
            data: Arc::new(RwLock::new(data)),
        }
    }

    /// Read data from the buffer. Multiple consumers can read concurrently.
    fn read(&self) -> Option<Vec<u8>> {
        self.data.read().ok().map(|guard| guard.clone())
    }

    /// Update the buffer's data. Only one writer is allowed at a time.
    fn write(&self, new_data: Vec<u8>) -> Result<(), String> {
        self.data
            .write()
            .map(|mut guard| {
                *guard = new_data;
            })
            .map_err(|_| "Failed to acquire write lock".to_string())
    }
}

fn main() {
    // Create a new Zero-Copy Buffer with initial data
    let buffer = ZeroCopyBuffer::new(vec![1, 2, 3, 4, 5]);

    // Multiple readers
    let reader1 = buffer.clone();
    let reader2 = buffer.clone();

    // Spawn threads to simulate concurrent reads
    let handle1 = std::thread::spawn(move || {
        if let Some(data) = reader1.read() {
            println!("Reader 1: {:?}", data);
        } else {
            println!("Reader 1: Failed to read data");
        }
    });

    let handle2 = std::thread::spawn(move || {
        if let Some(data) = reader2.read() {
            println!("Reader 2: {:?}", data);
        } else {
            println!("Reader 2: Failed to read data");
        }
    });

    // Wait for readers to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Update the buffer's data
    if let Err(err) = buffer.write(vec![6, 7, 8, 9, 10]) {
        println!("Writer: {}", err);
    } else {
        println!("Writer: Updated the buffer");
    }

    // Verify updated data
    if let Some(data) = buffer.read() {
        println!("Main Thread: Updated Data: {:?}", data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path_read_write() {
        let buffer = ZeroCopyBuffer::new(vec![1, 2, 3, 4, 5]);

        // Read initial data
        assert_eq!(buffer.read(), Some(vec![1, 2, 3, 4, 5]));

        // Update the buffer's data
        assert!(buffer.write(vec![10, 20, 30, 40, 50]).is_ok());

        // Read updated data
        assert_eq!(buffer.read(), Some(vec![10, 20, 30, 40, 50]));
    }

    #[test]
    fn test_unhappy_path_write_lock_failure() {
        let buffer = ZeroCopyBuffer::new(vec![1, 2, 3, 4, 5]);
    
        // Hold a write lock in one thread
        let buffer_clone = buffer.clone();
        let writer_thread = std::thread::spawn(move || {
            let _write_lock = buffer_clone.data.write().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(2)); // Hold the lock for a while
        });
    
        // Give the first thread time to acquire the lock
        std::thread::sleep(std::time::Duration::from_millis(500));
    
        // Attempt to acquire a write lock in the main thread
        let result = buffer.data.try_write();
    
        writer_thread.join().unwrap(); // Ensure the first thread finishes
    
        // Check if the write lock failed to acquire
        assert!(result.is_err(), "Expected a lock contention error, but lock succeeded");
    }    

    #[test]
    fn test_concurrent_reads() {
        let buffer = ZeroCopyBuffer::new(vec![1, 2, 3, 4, 5]);

        // Simulate concurrent reads
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let reader = buffer.clone();
                std::thread::spawn(move || reader.read())
            })
            .collect();

        for handle in handles {
            let result = handle.join().unwrap();
            assert_eq!(result, Some(vec![1, 2, 3, 4, 5]));
        }
    }

    #[test]
    fn test_empty_buffer_read() {
        let buffer = ZeroCopyBuffer::new(Vec::new());
        assert_eq!(buffer.read(), Some(Vec::new()));
    }
}
