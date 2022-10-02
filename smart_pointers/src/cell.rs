use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

// Remove all safety measures for Cell<T> to show test cases
unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

mod test {
    use super::Cell;
    use std::sync::Arc;

    // Multi-threading example
    #[test]
    fn bad() {
        let x = Arc::new(Cell::new(42));

        let x1 = Arc::clone(&x);
        std::thread::spawn(move || {
            x1.set(43);
        });

        // x should have been invalidated but it is not
        let x2 = Arc::clone(&x);
        std::thread::spawn(move || {
            x2.set(44);
        });
    }
}
