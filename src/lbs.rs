use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    count: Mutex<i64>,
    cvar: Condvar,
}

impl Semaphore {
    pub fn new(count: i64) -> Self {
        Semaphore {
            count: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.cvar.notify_one();
    }
}
