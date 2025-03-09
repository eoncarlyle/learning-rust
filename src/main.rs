use std::sync::Arc;
use std::thread::{self};
mod lbs;

fn main() {
    problem_3_1();
}

fn problem_3_1() {
    let sem = Arc::new(lbs::Semaphore::new(0));
    let sem_clone0 = Arc::clone(&sem);
    let sem_clone1 = Arc::clone(&sem);

    let handle0 = thread::spawn(move || {
        println!("Statement a1");
        sem_clone0.release();
    });

    let handle1 = thread::spawn(move || {
        sem_clone1.acquire();
        println!("Statement b1");
    });

    handle0.join().unwrap();
    handle1.join().unwrap();
}
