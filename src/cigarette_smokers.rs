use crate::lbs::Semaphore;
use std::sync::atomic::{self, AtomicBool};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time};

enum ResourceOwned {
    Tobacco,
    Paper,
    Lighter,
}

struct AgentBools {
    first_set_true: Arc<AtomicBool>,
    second_set_true: Arc<AtomicBool>,
    set_false: Arc<AtomicBool>,
}

struct ConsumerArcs {
    first_sem: Arc<Semaphore>,
    second_sem: Arc<Semaphore>,
    first_bool: Arc<AtomicBool>,
    second_bool: Arc<AtomicBool>,
}

pub fn problem_4_5() {
    fn agent_thread(
        label: String,
        agent_sem: Arc<Semaphore>,
        first_signaled_sem: Arc<Semaphore>,
        second_signaled_sem: Arc<Semaphore>,
        agent_bools: AgentBools,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            let AgentBools {
                first_set_true,
                second_set_true,
                set_false,
            } = agent_bools;

            loop {
                agent_sem.acquire();
                first_signaled_sem.release();
                second_signaled_sem.release();

                first_set_true.store(true, atomic::Ordering::Relaxed);
                second_set_true.store(false, atomic::Ordering::Relaxed);
                set_false.store(false, atomic::Ordering::Relaxed);

                println!("agent {label} run");
                thread::sleep(time::Duration::from_millis(400));
            }
        });
    }

    fn consumer_thread(
        label: String,
        agent_sem: Arc<Semaphore>,
        consumer_arcs: ConsumerArcs,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            let ConsumerArcs {
                first_sem,
                second_sem,
                first_bool,
                second_bool,
            } = consumer_arcs;

            let mut ineligible = true;

            while ineligible {
                first_sem.acquire();
                second_sem.acquire();
                if first_bool.load(atomic::Ordering::Acquire)
                    && second_bool.load(atomic::Ordering::Acquire)
                {
                    ineligible = false
                } else {
                    first_sem.release();
                    second_sem.release();
                    thread::sleep(time::Duration::from_millis(400));
                }
            }

            first_bool.store(false, atomic::Ordering::Relaxed);
            second_bool.store(false, atomic::Ordering::Relaxed);

            println!("{label} consumer run");
            first_sem.release();
            second_sem.release();
            agent_sem.release();
        });
    }

    let is_tobbacco = Arc::new(AtomicBool::new(false));
    let is_paper = Arc::new(AtomicBool::new(false));
    let is_lighter = Arc::new(AtomicBool::new(false));

    let agent_sem = Arc::new(Semaphore::new(1));
    let tobacco = Arc::new(Semaphore::new(0));
    let paper = Arc::new(Semaphore::new(0));
    let lighter = Arc::new(Semaphore::new(0));

    let agent_a = agent_thread(
        String::from("a"),
        agent_sem.clone(),
        tobacco.clone(),
        paper.clone(),
        AgentBools {
            first_set_true: is_tobbacco.clone(),
            second_set_true: is_paper.clone(),
            set_false: is_lighter.clone(),
        },
    );

    agent_thread(
        String::from("b"),
        agent_sem.clone(),
        paper.clone(),
        lighter.clone(),
        AgentBools {
            first_set_true: is_paper.clone(),
            second_set_true: is_lighter.clone(),
            set_false: is_tobbacco.clone(),
        },
    );

    agent_thread(
        String::from("c"),
        agent_sem.clone(),
        lighter.clone(),
        tobacco.clone(),
        AgentBools {
            first_set_true: is_lighter.clone(),
            second_set_true: is_tobbacco.clone(),
            set_false: is_paper.clone(),
        },
    );

    consumer_thread(
        String::from("lighter"),
        agent_sem.clone(),
        ConsumerArcs {
            first_sem: tobacco.clone(),
            second_sem: paper.clone(),
            first_bool: is_tobbacco.clone(),
            second_bool: is_paper.clone(),
        },
    );

    consumer_thread(
        String::from("tobacco"),
        agent_sem.clone(),
        ConsumerArcs {
            first_sem: paper.clone(),
            second_sem: lighter.clone(),
            first_bool: is_paper.clone(),
            second_bool: is_lighter.clone(),
        },
    );

    consumer_thread(
        String::from("paper"),
        agent_sem.clone(),
        ConsumerArcs {
            first_sem: tobacco.clone(),
            second_sem: lighter.clone(),
            first_bool: is_tobbacco.clone(),
            second_bool: is_lighter.clone(),
        },
    );

    agent_a.join();
}
