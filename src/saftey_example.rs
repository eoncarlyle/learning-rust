use crate::lbs::Semaphore;
use parking_lot::Mutex as PLMutex;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn problem_3_8_thread(
    internal_turnstile: Arc<Semaphore>,
    external_turnstile: Arc<Semaphore>,
    dancer_list: Arc<Mutex<LinkedList<String>>>,
    label: String,
) -> JoinHandle<()> {
    return thread::spawn(move || {
        let mut is_empty = true;

        {
            let mut dancer_list_data = dancer_list.lock().unwrap();
            let data = *dancer_list_data;
            is_empty = data.is_empty();
        }

        while !is_empty {
            println!("{label} thread waiting");
            internal_turnstile.release();
            external_turnstile.acquire();

            {
                let mut dancer_list_data = dancer_list.lock().unwrap();
                let mut data = *dancer_list_data;
                let a = data.pop_front();
                a.map(|b| println!("{b}"));
            }
        }
    });
}

pub fn problem_3_8() {
    let leader_semaphore = Arc::new(Semaphore::new(0));
    let follow_semaphore = Arc::new(Semaphore::new(0));

    let leader_list = Arc::new(Mutex::new(LinkedList::new()));

    {
        let mut leader_list_data = leader_list.lock().unwrap();

        leader_list_data.push_back(String::from("leader1"));
        leader_list_data.push_back(String::from("leader2"));
        leader_list_data.push_back(String::from("leader3"));
        leader_list_data.push_back(String::from("leader4"));
    }

    let leader_handle = problem_3_8_thread(
        leader_semaphore.clone(),
        follow_semaphore.clone(),
        leader_list.clone(),
        String::from("leader"),
    );

    let follower_list = Arc::new(Mutex::new(LinkedList::new()));

    let follower_handle = problem_3_8_thread(
        follow_semaphore.clone(),
        leader_semaphore.clone(),
        follower_list.clone(),
        String::from("follower"),
    );

    {
        let mut follower_list_data = follower_list.lock().unwrap();
        follower_list_data.push_back(String::from("follower1"));
        follower_list_data.push_back(String::from("follower2"));
    }
    leader_handle.join().unwrap();
    follower_handle.join().unwrap();
}
