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
        first_waited_sem: Arc<Semaphore>,
        second_waited_sem: Arc<Semaphore>,
        agent_sem: Arc<Semaphore>,
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

    let mut rng = rand::rng();

    let agent_sem = Arc::new(Semaphore::new(1));
    let tobacco = Arc::new(Semaphore::new(0));
    let paper = Arc::new(Semaphore::new(0));
    let lighter = Arc::new(Semaphore::new(0));

    fn release_resources(
        first_rand_val: i64,
        second_rand_val: i64,
        tobacco: Arc<Semaphore>,
        paper: Arc<Semaphore>,
        lighter: Arc<Semaphore>,
    ) {
        for rand_val in vec![first_rand_val, second_rand_val] {
            match rand_val {
                1 => tobacco.release(),
                2 => paper.release(),
                _ => lighter.release(),
            }
        }
    }

    let agent_a = agent_thread(
        String::from("a"),
        agent_sem.clone(),
        tobacco.clone(),
        paper.clone(),
    );

    let agent_b = agent_thread(
        String::from("b"),
        agent_sem.clone(),
        paper.clone(),
        lighter.clone(),
    );

    let agent_c = agent_thread(
        String::from("c"),
        agent_sem.clone(),
        lighter.clone(),
        tobacco.clone(),
    );

    let consumer_a = consumer_thread(
        String::from("a"),
        tobacco.clone(),
        paper.clone(),
        agent_sem.clone(),
    );
    let consumer_b = consumer_thread(
        String::from("b"),
        paper.clone(),
        lighter.clone(),
        agent_sem.clone(),
    );
    let consumer_c = consumer_thread(
        String::from("c"),
        tobacco.clone(),
        lighter.clone(),
        agent_sem,
    );

    loop {
        let first_rand_val = rng.random_range(0..=3);
        let second_rand_val = rng.random_range(0..=3);

        release_resources(
            first_rand_val,
            second_rand_val,
            tobacco.clone(),
            paper.clone(),
            lighter.clone(),
        );

        thread::sleep(time::Duration::from_millis(400));
    }
}
