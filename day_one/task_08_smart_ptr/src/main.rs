use std::cell::{Ref, RefCell, RefMut};

/// A custom smart pointer with interior mutability.
pub struct CustomSmartPointer<T> {
    value: RefCell<T>,
}

impl<T> CustomSmartPointer<T> {
    /// Creates a new instance of the custom smart pointer.
    pub fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
        }
    }

    /// Explicitly borrow the inner value immutably.
    pub fn borrow(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    /// Explicitly borrow the inner value mutably.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.value.borrow_mut()
    }
}

fn main() {
    // Create a CustomSmartPointer with an initial value.
    let smart_pointer = CustomSmartPointer::new(10);

    // Immutable borrow.
    {
        let borrowed_value = smart_pointer.borrow();
        println!("Immutable borrow: {}", *borrowed_value);
        // The borrowed_value goes out of scope here, allowing further borrows.
    }

    // Mutable borrow.
    {
        let mut borrowed_mut = smart_pointer.borrow_mut();
        *borrowed_mut = 20;
        println!("Mutable borrow (inside scope): {}", *borrowed_mut);
        // The borrowed_mut goes out of scope here, allowing further borrows.
    }

    // Immutable borrow after mutation.
    println!("Immutable borrow after mutation: {}", *smart_pointer.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_access() {
        let smart_pointer = CustomSmartPointer::new(42);
        let borrowed = smart_pointer.borrow();
        assert_eq!(*borrowed, 42);
    }

    #[test]
    fn test_mutable_access() {
        let smart_pointer = CustomSmartPointer::new(42);
        {
            let mut borrowed_mut = smart_pointer.borrow_mut();
            *borrowed_mut = 100;
        }
        let borrowed = smart_pointer.borrow();
        assert_eq!(*borrowed, 100);
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_runtime_borrow_violation() {
        let smart_pointer = CustomSmartPointer::new(42);
        let _borrowed_immutable = smart_pointer.borrow();
        // This will cause a runtime panic due to a violation of borrowing rules.
        let _borrowed_mutable = smart_pointer.borrow_mut();
    }
}
