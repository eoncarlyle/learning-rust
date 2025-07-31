use std::sync::atomic::{AtomicU8, Ordering};
use std::thread::{self, JoinHandle};

use std::sync::Arc;
use std::time::Duration;

use crate::lbs::Semaphore;

pub fn run() {
    let hair_cut_request = Arc::new(Semaphore::new(1));
    let hair_cut_response = Arc::new(Semaphore::new(0));
    let free_chairs = Arc::new(AtomicU8::new(5));
    let mutex_sem = Arc::new(Semaphore::new(1));

    fn get_hair_cut(
        label: String,
        hair_cut_request: Arc<Semaphore<i32>>,
        hair_cut_response: Arc<Semaphore<i32>>,
    ) {
        hair_cut_request.release();
        hair_cut_response.acquire();

        println!("Consumer {label} recieved haircut");
    }
    fn cut_hair() {
        println!("Barber cutting hair");
    }

    fn balk() {}

    fn barber_thread(
        hair_cut_request: Arc<Semaphore<i32>>,
        hair_cut_response: Arc<Semaphore<i32>>,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                hair_cut_request.acquire();
                cut_hair();
                hair_cut_response.release();
                thread::sleep(Duration::from_millis(400));
            }
        });
    }
    fn customer(
        label: String,
        hair_cut_request: Arc<Semaphore<i32>>,
        hair_cut_response: Arc<Semaphore<i32>>,
        free_chairs: Arc<AtomicU8>,
        mutex_sem: Arc<Semaphore<i32>>,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                mutex_sem.acquire();
                let mut has_spot = false;
                if free_chairs.load(Ordering::SeqCst) == 0 {
                    balk();
                } else {
                    has_spot = true;
                    free_chairs.fetch_sub(1, Ordering::SeqCst);
                }
                mutex_sem.release();

                if has_spot {
                    get_hair_cut(
                        label.clone(),
                        hair_cut_request.clone(),
                        hair_cut_response.clone(),
                    );
                    mutex_sem.acquire();
                    free_chairs.fetch_add(1, Ordering::SeqCst);
                    mutex_sem.release();
                }
            }
        });
    }

    (0..10).for_each(|l| {
        customer(
            l.to_string(),
            hair_cut_request.clone(),
            hair_cut_response.clone(),
            free_chairs.clone(),
            mutex_sem.clone(),
        );
    });

    barber_thread(hair_cut_request, hair_cut_response).join();
}
