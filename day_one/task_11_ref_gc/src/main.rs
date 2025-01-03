use std::rc::Rc;
use std::cell::RefCell;

pub mod arc; 

#[derive(Debug)]
struct MyData {
    value: i32,
}

impl MyData {
    fn new(value: i32) -> Self {
        MyData { value }
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

#[derive(Debug)]
struct ReferenceCountedGC {
    data: Rc<RefCell<MyData>>,
}

impl ReferenceCountedGC {
    fn new(value: i32) -> Self {
        let data = Rc::new(RefCell::new(MyData::new(value)));
        ReferenceCountedGC { data }
    }

    fn get_data(&self) -> Rc<RefCell<MyData>> {
        Rc::clone(&self.data)
    }
        
    // Rust will automatically clean up when no references exist, 
}

fn main() {
    let gc = ReferenceCountedGC::new(42);

    // Create multiple references using Rc
    let data_ref1 = gc.get_data();
    let data_ref2 = gc.get_data();

    println!("Data via reference 1: {}", data_ref1.borrow().get_value());
    println!("Data via reference 2: {}", data_ref2.borrow().get_value());

    // After this point, both references are still valid, and the memory is not freed.
    // Once the references are dropped, the memory will be freed automatically.

    drop(data_ref1);  // Dropping the first reference.
    println!("Reference count after dropping one reference: {}",
             Rc::strong_count(&gc.data));

    // Once both references go out of scope, `MyData` will be deallocated automatically.

    drop(data_ref2);  // Dropping the second reference.
    println!("Reference count after dropping second reference: {}",
             Rc::strong_count(&gc.data));

    // gc will be cleaned up at the end of main, when no more references remain.

    println!("Running arc example...");
    arc::run_arc_example();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_successfully() {
        let gc = ReferenceCountedGC::new(100);
        let data_ref = gc.get_data();
        
        assert_eq!(data_ref.borrow().get_value(), 100, "Value should be 100");
        assert_eq!(Rc::strong_count(&gc.data), 2, "Reference count should be 2 after cloning");
    }

    #[test]
    fn test_reference_counting() {
        let gc = ReferenceCountedGC::new(200);
        let data_ref1 = gc.get_data();
        let data_ref2 = gc.get_data();
        
        assert_eq!(Rc::strong_count(&gc.data), 3, "Reference count should be 3 after cloning twice");

        drop(data_ref1);
        assert_eq!(Rc::strong_count(&gc.data), 2, "Reference count should decrease after dropping one reference");

        drop(data_ref2);
        assert_eq!(Rc::strong_count(&gc.data), 1, "Reference count should decrease after dropping the second reference");
    }

    #[test]
    fn test_cleanup_when_no_references_left() {
        let gc = ReferenceCountedGC::new(500);

        {
            let data_ref1 = gc.get_data();
            assert_eq!(Rc::strong_count(&gc.data), 2, "Reference count should be 2");

            drop(data_ref1); // Drop inside block
        }

        // No more references exist, the memory is automatically cleaned up
        assert_eq!(Rc::strong_count(&gc.data), 1, "Reference count should be 1 after dropping the reference inside block");
    }

    #[test]
    fn test_gc_behavior_with_multiple_refs() {
        let gc = ReferenceCountedGC::new(1000);

        let data_ref1 = gc.get_data();
        let data_ref2 = gc.get_data();
        let data_ref3 = gc.get_data();

        assert_eq!(Rc::strong_count(&gc.data), 4, "Reference count should be 4 after creating 3 references");

        drop(data_ref1);
        drop(data_ref2);

        assert_eq!(Rc::strong_count(&gc.data), 2, "Reference count should be 2 after dropping two references");

        drop(data_ref3);

        // The reference count is now 1 because `gc` still holds the reference.
        assert_eq!(Rc::strong_count(&gc.data), 1, "Reference count should be 1 when all external references are dropped");

        // Once gc goes out of scope, the memory will be freed automatically.
    }

    #[test]
    fn test_multiple_references_dropped_in_order() {
        let gc = ReferenceCountedGC::new(300);

        let data_ref1 = gc.get_data();
        let data_ref2 = gc.get_data();

        assert_eq!(Rc::strong_count(&gc.data), 3, "Reference count should be 3 after creating two references");

        // Drop references in reverse order
        drop(data_ref2);
        assert_eq!(Rc::strong_count(&gc.data), 2, "Reference count should be 2 after dropping second reference");

        drop(data_ref1);
        assert_eq!(Rc::strong_count(&gc.data), 1, "Reference count should be 1 after dropping first reference");

        // gc will be cleaned up once it goes out of scope, memory is freed automatically.
    }

    #[test]
    fn test_gc_behavior_with_no_references() {
        let gc = ReferenceCountedGC::new(700);

        // No references are created; memory will be cleaned up once gc goes out of scope.
        assert_eq!(Rc::strong_count(&gc.data), 1, "Reference count should be 1 when no references are created");
    }
}



#[cfg(test)]
mod arc_tests {
    use crate::arc::ReferenceCountedGC as ArcReferenceCountedGC;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_allocate_successfully() {
        let gc = ArcReferenceCountedGC::new(100);
        let data_ref = gc.get_data();
        
        assert_eq!(data_ref.lock().unwrap().get_value(), 100, "Value should be 100");
    }

    #[test]
    fn test_reference_counting() {
        let gc = ArcReferenceCountedGC::new(200);
        let data_ref1 = gc.get_data();
        let data_ref2 = gc.get_data();
        
        assert_eq!(Arc::strong_count(&gc.data), 3, "Reference count should be 3 after cloning twice");

        drop(data_ref1);
        assert_eq!(Arc::strong_count(&gc.data), 2, "Reference count should decrease after dropping one reference");

        drop(data_ref2);
        assert_eq!(Arc::strong_count(&gc.data), 1, "Reference count should decrease after dropping the second reference");
    }

    #[test]
    fn test_cleanup_when_no_references_left() {
        let gc = ArcReferenceCountedGC::new(500);

        {
            let data_ref1 = gc.get_data();
            assert_eq!(Arc::strong_count(&gc.data), 2, "Reference count should be 2");

            drop(data_ref1); // Drop inside block
        }

        // No more references exist, the memory is automatically cleaned up
        assert_eq!(Arc::strong_count(&gc.data), 1, "Reference count should be 1 after dropping the reference inside block");
    }

    #[test]
    fn test_gc_behavior_with_multiple_refs() {
        let gc = ArcReferenceCountedGC::new(1000);

        let data_ref1 = gc.get_data();
        let data_ref2 = gc.get_data();
        let data_ref3 = gc.get_data();

        assert_eq!(Arc::strong_count(&gc.data), 4, "Reference count should be 4 after creating 3 references");

        drop(data_ref1);
        drop(data_ref2);

        assert_eq!(Arc::strong_count(&gc.data), 2, "Reference count should be 2 after dropping two references");

        drop(data_ref3);

        // The reference count is now 1 because `gc` still holds the reference.
        assert_eq!(Arc::strong_count(&gc.data), 1, "Reference count should be 1 when all external references are dropped");

        // Once gc goes out of scope, the memory will be freed automatically.
    }

    #[test]
    fn test_gc_with_threads() {
        let gc = ArcReferenceCountedGC::new(42);
        let data_ref = gc.get_data();

        let handle1 = thread::spawn({
            let data_ref1 = Arc::clone(&data_ref); // Clone the Arc before moving it into the thread
            move || {
                assert_eq!(data_ref1.lock().unwrap().get_value(), 42, "Thread 1 should see the correct value");
            }
        });

        let handle2 = thread::spawn({
            let data_ref2 = Arc::clone(&data_ref); // Clone the Arc before moving it into the thread
            move || {
                assert_eq!(data_ref2.lock().unwrap().get_value(), 42, "Thread 2 should see the correct value");
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        // Test memory cleanup after thread execution
        drop(data_ref);
        assert_eq!(Arc::strong_count(&gc.data), 1, "Reference count should be 1 after dropping references");

        // No references remaining, and memory is automatically freed.
    }
}
