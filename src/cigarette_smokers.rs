use crate::lbs::Semaphore;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{i64, thread, time};

pub fn problem_4_5_1() {
    fn agent_thread(
        label: String,
        agent_sem: Arc<Semaphore>,
        first_signaled_sem: Arc<Semaphore>,
        second_signaled_sem: Arc<Semaphore>,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                agent_sem.acquire();
                first_signaled_sem.release();
                second_signaled_sem.release();
                println!("agent {label} run");
                thread::sleep(time::Duration::from_millis(400));
            }
        });
    }

    fn consumer_thread(
        label: String,
        agent_sem: Arc<Semaphore>,
        first_waited_sem: Arc<Semaphore>,
        second_waited_sem: Arc<Semaphore>,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                first_waited_sem.acquire();
                second_waited_sem.acquire();
                agent_sem.release();
                println!("consumer {label} run");
                thread::sleep(time::Duration::from_millis(400));
            }
        });
    }

    let agent_sem = Arc::new(Semaphore::new(1));
    let tobacco = Arc::new(Semaphore::new(0));
    let paper = Arc::new(Semaphore::new(0));
    let lighter = Arc::new(Semaphore::new(0));

    let agent_a = agent_thread(
        String::from("a"),
        agent_sem.clone(),
        tobacco.clone(),
        paper.clone(),
    );

    agent_thread(
        String::from("b"),
        agent_sem.clone(),
        paper.clone(),
        lighter.clone(),
    );

    agent_thread(
        String::from("c"),
        agent_sem.clone(),
        lighter.clone(),
        tobacco.clone(),
    );

    consumer_thread(
        String::from("a"),
        agent_sem.clone(),
        tobacco.clone(),
        paper.clone(),
    );

    consumer_thread(
        String::from("b"),
        agent_sem.clone(),
        paper.clone(),
        lighter.clone(),
    );

    consumer_thread(
        String::from("c"),
        agent_sem.clone(),
        tobacco.clone(),
        lighter.clone(),
    );

    agent_a.join().unwrap();
}
