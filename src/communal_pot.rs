// As written in Chapter 5 of Downey the classical name is different
use crate::lbs::Semaphore;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::u8::MAX;

pub fn run() {
    const MAX_SERVINGS: u8 = 4;
    let serving_count = Arc::new(AtomicU8::new(MAX_SERVINGS));
    let mutex_sem = Arc::new(Semaphore::new(1));
    let refill_sem = Arc::new(Semaphore::new(0));

    fn cook_thread(
        mutex_sem: Arc<Semaphore>,
        refill_sem: Arc<Semaphore>,
        serving_count: Arc<AtomicU8>,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                refill_sem.acquire();
                println!("Cook awaken");

                mutex_sem.acquire();
                serving_count.store(MAX_SERVINGS, Ordering::Relaxed);
                mutex_sem.release();

                println!("Cook done");
                thread::sleep(Duration::from_millis(400));
            }
        });
    }

    fn diner_thread(
        label: String,
        mutex_sem: Arc<Semaphore>,
        refill_sem: Arc<Semaphore>,
        serving_count: Arc<AtomicU8>,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                mutex_sem.acquire();
                let count = serving_count.load(Ordering::Relaxed);
                if count == 0 {
                    println!("Diner {label} signaling cook");
                    serving_count.store(MAX_SERVINGS + 1, Ordering::Relaxed);
                    refill_sem.release();
                } else if count == MAX_SERVINGS + 1 {
                    println!("Diner {label} waiting for cook");
                } else {
                    println!("Diner {label} consuming");
                    serving_count.store(count - 1, Ordering::Relaxed);
                }
                mutex_sem.release();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    let cook = cook_thread(mutex_sem.clone(), refill_sem.clone(), serving_count.clone());

    ["a", "b", "c"].map(|label| {
        diner_thread(
            String::from(label),
            mutex_sem.clone(),
            refill_sem.clone(),
            serving_count.clone(),
        );
    });

    cook.join();
}
