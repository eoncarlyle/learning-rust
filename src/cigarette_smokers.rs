use crate::lbs::Semaphore;
use std::sync::atomic::{self, AtomicBool};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time};

type Callback = fn() -> ();

enum ResourceOwned {
    Tobacco,
    Paper,
    Lighter,
}

fn problem_4_5() {
    fn agent_thread(
        label: String,
        agent_sem: Arc<Semaphore>,
        first_signaled_sem: Arc<Semaphore>,
        second_signaled_sem: Arc<Semaphore>,
        cb: Callback,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                agent_sem.acquire();
                first_signaled_sem.release();
                second_signaled_sem.release();
                cb();
                println!("agent {label} run");
                thread::sleep(time::Duration::from_millis(400));
            }
        });
    }

    // TODO: pass only the semaphores and variables required to the thread
    fn consumer_thread(
        resource_owned: ResourceOwned,
        label: String,
        agent_sem: Arc<Semaphore>,
        resource_sems: &[Arc<Semaphore>],
        resource_vars: &[Arc<AtomicBool>],
    ) {
        let vars = resource_vars.to_vec();
        let sems = resource_sems.to_vec();
        match resource_owned {
            ResourceOwned::Tobacco => {}
            ResourceOwned::Paper => {}
            ResourceOwned::Lighter => {}
        }
    }

    let is_tobbacco = Arc::new(AtomicBool::new(false));
    let is_paper = Arc::new(AtomicBool::new(false));
    let is_lighter = Arc::new(AtomicBool::new(false));

    let agent_sem = Arc::new(Semaphore::new(1));
    let tobacco = Arc::new(Semaphore::new(0));
    let paper = Arc::new(Semaphore::new(0));
    let lighter = Arc::new(Semaphore::new(0));

    fn make_callback(set_true: &[Arc<AtomicBool>], set_false: &[Arc<AtomicBool>]) -> Callback {
        let set_true = set_true.to_vec();
        let set_false = set_false.to_vec();

        move || {
            for item in &set_true {
                item.store(true, atomic::Ordering::Relaxed);
            }
            for item in &set_false {
                item.store(false, atomic::Ordering::Relaxed);
            }
        }
    }

    let agent_a = agent_thread(
        String::from("a"),
        agent_sem.clone(),
        tobacco.clone(),
        paper.clone(),
        make_callback(
            &[is_tobbacco.clone(), is_paper.clone()],
            &[is_lighter.clone()],
        ),
    );

    let agent_b = agent_thread(
        String::from("b"),
        agent_sem.clone(),
        paper.clone(),
        lighter.clone(),
        make_callback(
            &[is_paper.clone(), is_lighter.clone()],
            &[is_tobbacco.clone()],
        ),
    );

    let agent_c = agent_thread(
        String::from("c"),
        agent_sem.clone(),
        lighter.clone(),
        tobacco.clone(),
        make_callback(
            &[is_lighter.clone(), is_tobbacco.clone()],
            &[is_paper.clone()],
        ),
    );
}
