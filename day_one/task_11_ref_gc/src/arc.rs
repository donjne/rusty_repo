use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct MyData {
    pub value: i32,
}

impl MyData {
    pub fn new(value: i32) -> Self {
        MyData { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[derive(Debug)]
pub struct ReferenceCountedGC {
    pub data: Arc<Mutex<MyData>>,
}

impl ReferenceCountedGC {
    pub fn new(value: i32) -> Self {
        let data = Arc::new(Mutex::new(MyData::new(value)));
        ReferenceCountedGC { data }
    }

    pub fn get_data(&self) -> Arc<Mutex<MyData>> {
        Arc::clone(&self.data)
    }
    
    // The memory will be freed when the last reference to `Arc` is dropped.
}

pub fn run_arc_example() {
    let gc = ReferenceCountedGC::new(42);

    // Create multiple references using Arc
    let data_ref1 = gc.get_data();
    let data_ref2 = gc.get_data();

    println!("Data via reference 1: {}", data_ref1.lock().unwrap().get_value());
    println!("Data via reference 2: {}", data_ref2.lock().unwrap().get_value());

    // Now let's spawn threads and share the data between them using Arc
    let handle1 = thread::spawn({
        let data_ref1 = Arc::clone(&data_ref1); // Clone the Arc before moving it into the thread
        move || {
            println!("Thread 1: {}", data_ref1.lock().unwrap().get_value());
        }
    });

    let handle2 = thread::spawn({
        let data_ref2 = Arc::clone(&data_ref2); // Clone the Arc before moving it into the thread
        move || {
            println!("Thread 2: {}", data_ref2.lock().unwrap().get_value());
        }
    });

    // Wait for threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();

    // The memory is freed automatically once both references are dropped and no longer in scope
    drop(data_ref1);
    drop(data_ref2);

    println!("Memory will be cleaned up when the last reference goes out of scope.");
}

