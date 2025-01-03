use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct CopyOnWrite<T>
where
    T: Clone,
{
    inner: Arc<RwLock<Arc<T>>>,
}

impl<T> CopyOnWrite<T>
where
    T: Clone,
{
    /// Create a new CopyOnWrite instance.
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(Arc::new(data))),
        }
    }

    /// Read the current data.
    pub fn read(&self) -> Arc<T> {
        self.inner.read().unwrap().clone()
    }

    /// Write new data (cloning only if necessary).
    pub fn write(&self, modify_fn: impl FnOnce(&mut T)) {
        let mut lock = self.inner.write().unwrap();
        let mut_data = Arc::make_mut(&mut lock);
        modify_fn(mut_data);
    }
}

fn main() {
    let cow = CopyOnWrite::new(vec![1, 2, 3, 4, 5]);

    // Read data
    let data = cow.read();
    println!("Initial data: {:?}", data);

    // Modify data using write
    cow.write(|data| {
        data.push(6);
    });

    // Read the modified data
    let modified_data = cow.read();
    println!("Modified data: {:?}", modified_data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path_read() {
        let cow = CopyOnWrite::new(vec![1, 2, 3]);
        let data = cow.read();
        assert_eq!(*data, vec![1, 2, 3]);
    }

    #[test]
    fn test_happy_path_write() {
        let cow = CopyOnWrite::new(vec![1, 2, 3]);
        cow.write(|data| {
            data.push(4);
        });
        let data = cow.read();
        assert_eq!(*data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_unhappy_path_write_with_failed_lock() {
        let cow = CopyOnWrite::new(vec![1, 2, 3]);
    
        // Simulate a lock poisoning scenario
        let poisoned_lock = cow.inner.clone();
        std::thread::spawn(move || {
            drop(poisoned_lock.write().unwrap()); // Explicitly drop the lock
            panic!("Simulated lock failure");
        })
        .join()
        .unwrap_err();
    
        // Verify the lock is functional again
        cow.write(|data| {
            data.push(4);
        });
        let data = cow.read();
        assert_eq!(*data, vec![1, 2, 3, 4]);
    }    

    #[test]
    fn test_edge_case_empty_data() {
        let cow = CopyOnWrite::new(Vec::<i32>::new());
        assert!(cow.read().is_empty());

        cow.write(|data| {
            data.push(42);
        });
        let data = cow.read();
        assert_eq!(*data, vec![42]);
    }

    #[test]
    fn test_edge_case_large_data() {
        let large_data: Vec<i32> = (0..10_000).collect();
        let cow = CopyOnWrite::new(large_data.clone());
        assert_eq!(*cow.read(), large_data);

        cow.write(|data| {
            data.push(10_001);
        });
        let mut expected = large_data.clone();
        expected.push(10_001);
        assert_eq!(*cow.read(), expected);
    }
}
