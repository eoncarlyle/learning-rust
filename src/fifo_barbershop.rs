use std::sync::Arc;
use std::sync::atomic::{AtomicI32, AtomicUsize};
use std::sync::atomic::Ordering::SeqCst;
use crate::lbs::Semaphore;
use std::sync::mpsc::{channel, Sender};
use std::thread::JoinHandle;
use std::thread;


pub fn run() {
    let chair_count = 5;
    let chair_semaphore = Arc::new(Semaphore::new(chair_count));
    let chair_scoreboard = Arc::new(AtomicI32::new(5));
    let barber_semaphore = Arc::new(Semaphore::new(1));

    let chair_sems: Vec<_> = (0..chair_count).into_iter().map(|_| Arc::new(Semaphore::new(1))).collect();

    let (tx, rx) = channel::<i32>();

    fn balk(label: String) {
        println!("Customer {} balked", label);
    }

    fn customer(
        label: String,
        chair_scoreboard: Arc<AtomicI32>,
        chair_sems: Vec<Arc<Semaphore>>,
        tx: Sender<i32>
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            if (chair_scoreboard.load(SeqCst)) >= 0 {
                balk(label);
            } else {
                for i in 0..chair_count {
                    i
                }
            }
        })
    }

}