use std::sync::{Condvar, Mutex};

pub struct Semaphore<T = i64> {
    count: Mutex<T>,
    cvar: Condvar,
}

impl<T> Semaphore<T>
where
    T: Copy + PartialEq + From<u8> + std::ops::SubAssign + std::ops::AddAssign,
{
    pub fn new(count: T) -> Self {
        Semaphore {
            count: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == T::from(0) {
            count = self.cvar.wait(count).unwrap();
        }
        *count -= T::from(1);
    }

    pub fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += T::from(1);
        self.cvar.notify_one();
    }
}
