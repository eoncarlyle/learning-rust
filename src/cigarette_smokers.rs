use crate::lbs::Semaphore;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time};

fn problem_4_5() {
    fn innerThread(
        agentSem: Arc<Semaphore>,
        waitedSem: Arc<Semaphore>,
        firstSignaled: Arc<Semaphore>,
        secondSignaled: Arc<Semaphore>,
    ) {
        agentSem.acquire();
        firstSignaled.release();
        secondSignaled.release();
    }

    let agentSem = Arc::new(Semaphore::new(1));
    let tobacco = Arc::new(Semaphore::new(0));
    let paper = Arc::new(Semaphore::new(0));
    let lighter = Arc::new(Semaphore::new(0));
}
