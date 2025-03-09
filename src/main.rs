use std::sync::Arc;
use std::thread;
mod lbs;

fn main() {
    let handle0 = thread::spawn(|| {
        println!("Statement a1");
        sem.release();
    });

    let handle1 = thread::spawn(|| {
        sem.acquire();
        println!("Statement b1");
    });

    handle0.join().unwrap();
    handle1.join().unwrap();
}
