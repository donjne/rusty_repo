use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CustomAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

impl CustomAllocator {
    fn now_allocated() -> usize {
        ALLOCATED.load(Ordering::Relaxed)
    }
}

unsafe impl GlobalAlloc for CustomAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let memory = std::alloc::alloc(layout);
        if !memory.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        }
        memory
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        std::alloc::dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

fn main() {
    // Example:
    let layout = Layout::from_size_align(1024, 8).unwrap();
    let ptr = unsafe { CustomAllocator.alloc(layout) };
    if !ptr.is_null() {
        println!("Allocated memory at: {:?}", ptr);
        println!("Current allocated bytes: {}", CustomAllocator::now_allocated());
        // Use the memory...
        unsafe { CustomAllocator.dealloc(ptr, layout) };
        println!("After deallocation, current allocated bytes: {}", CustomAllocator::now_allocated());
    } else {
        println!("Memory allocation failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Layout;

    #[test]
    fn test_happy_path() {
        let layout = Layout::from_size_align(1024, 8).unwrap();
        let ptr = unsafe { CustomAllocator.alloc(layout) };
        
        assert!(!ptr.is_null(), "Allocation should succeed");
        assert_eq!(CustomAllocator::now_allocated(), 1024, "Allocated size should match");
        
        unsafe {
            CustomAllocator.dealloc(ptr, layout);
        }
        
        assert_eq!(CustomAllocator::now_allocated(), 0, "After deallocation, allocated bytes should be zero");
    }

    #[test]
    fn test_unhappy_path() {
        // Attempt to create a layout with an enormous size
        let huge_layout = Layout::from_size_align(usize::MAX, 1);
    
        match huge_layout {
            Ok(layout) => {
                let ptr = unsafe { CustomAllocator.alloc(layout) };
                assert!(ptr.is_null(), "Allocation should fail for an enormous size");
                assert_eq!(CustomAllocator::now_allocated(), 0, "No memory should have been allocated on failure");
    
                // Deallocate should not panic even for a failed allocation
                unsafe {
                    CustomAllocator.dealloc(ptr, layout);
                }
            }
            Err(_) => {
                // Layout creation failed, which is expected for invalid sizes
                println!("Layout creation failed as expected for an enormous size");
            }
        }
    }    

    #[test]
    fn test_multiple_allocations() {
        let layout = Layout::from_size_align(1024, 8).unwrap();
        let mut pointers = Vec::new();

        for _ in 0..10 {
            let ptr = unsafe { CustomAllocator.alloc(layout) };
            assert!(!ptr.is_null(), "Each allocation should succeed");
            pointers.push(ptr);
        }

        assert_eq!(CustomAllocator::now_allocated(), 1024 * 10, "Total allocated size should match");

        for ptr in pointers {
            unsafe {
                CustomAllocator.dealloc(ptr, layout);
            }
        }

        assert_eq!(CustomAllocator::now_allocated(), 0, "After all deallocations, allocated bytes should be zero");
    }

    #[test]
    fn test_zero_sized_allocation() {
        let layout = Layout::from_size_align(0, 8).unwrap();
        let ptr = unsafe { CustomAllocator.alloc(layout) };
        
        assert!(!ptr.is_null(), "Allocation of zero-size should still return a non-null pointer");
        assert_eq!(CustomAllocator::now_allocated(), 0, "Zero-sized allocation should not change allocated bytes");

        unsafe {
            CustomAllocator.dealloc(ptr, layout);
        }
        
        assert_eq!(CustomAllocator::now_allocated(), 0, "Deallocation of zero-sized should not affect allocated bytes");
    }
}