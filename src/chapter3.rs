use crate::lbs::Semaphore;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time};

fn problem_3_1() {
    let sem = Arc::new(Semaphore::new(0));
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

/*
3.3  Rendezvous

Puzzle: Generalize the signal pattern so that it works both ways. Thread A has
to wait for Thread B and vice versa. In other words, given this code

            Thread A                          Thread B
     +----------------------+          +----------------------+
   1 | statement a1         |        1 | statement b1         |
   2 | statement a2         |        2 | statement b2         |
     +----------------------+          +----------------------+

we want to guarantee that a1 happens before b2 and b1 happens before a2. In
writing your solution, be sure to specify the names and initial values of your
semaphores (little hint there).
*/
fn problem_3_3() {
    let sem_b_done = Arc::new(Semaphore::new(0));
    let sem_a_done = Arc::new(Semaphore::new(0));

    let sem_a_done_clone_a = Arc::clone(&sem_a_done);
    let sem_b_done_clone_a = Arc::clone(&sem_b_done);
    let sem_a_done_clone_b = Arc::clone(&sem_a_done);
    let sem_b_done_clone_b = Arc::clone(&sem_b_done);

    let handle_a = thread::spawn(move || {
        println!("statement a1");
        sem_b_done_clone_a.release();
        sem_a_done_clone_a.acquire();
        println!("statement a2");
    });

    let handle_b = thread::spawn(move || {
        println!("statement b1");
        sem_a_done_clone_b.release();
        sem_b_done_clone_b.acquire();
        println!("statement b2");
    });

    handle_a.join().unwrap();
    handle_b.join().unwrap();
}

/*
3.3  Rendezvous

Puzzle: Add semaphores to the following example to enforce mutual exclu-
sion to the shared variable count

            Thread A                          Thread B
     +----------------------+          +----------------------+
   1 | count = count + 1    |        1 | count = count + 1    |
     +----------------------+          +----------------------+
*/
fn problem_3_4() {
    let mutex = Arc::new(Semaphore::new(1));
    let sem_clone_a = Arc::clone(&mutex);
    let sem_clone_b = Arc::clone(&mutex);

    let count = Arc::new(Mutex::new(0));
    let count_clone_a = Arc::clone(&count);
    let count_clone_b = Arc::clone(&count);

    let handle_a = thread::spawn(move || {
        sem_clone_a.acquire();
        println!("Incrementing in A");
        let mut inner_count = count_clone_a.lock().unwrap();
        println!("A old_count: {inner_count}");
        *inner_count += 1;
        println!("A new_count: {inner_count}");
        sem_clone_a.release();
    });

    let handle_b = thread::spawn(move || {
        sem_clone_b.acquire();
        println!("Incrementing in A");
        let mut inner_count = count_clone_b.lock().unwrap();
        println!("B old_count: {inner_count}");
        *inner_count += 1;
        println!("B new_count: {inner_count}");
        sem_clone_b.release();
    });

    handle_a.join().unwrap();
    handle_b.join().unwrap();
    let final_count = count.lock().unwrap();
    println!("{final_count}");
}

/*
3.6  Barrier

Puzzle: Generalize the rendezvous solution. Every thread should run the
following code:

            Barrier code
     +----------------------+
   1 | rendezvous            |
   2 | critical point        |
     +----------------------+

The synchronization requirement is that no thread executes critical point
until after all threads have executed rendezvous.
You can assume that there are n threads and that this value is stored in a
variable, n, that is accessible from all threads.
When the first n 1 threads arrive they should block until the nth thread
arrives, at which point all the threads may proceed.

*/
fn problem_3_6_thread(
    barrier: Arc<Semaphore>,
    count: Arc<Mutex<i64>>,
    thread_index: i64,
    thread_count: i64,
) -> JoinHandle<()> {
    //let sem_clones: Vec<Arc<Semaphore>> = sems.iter().map(|sem| Arc::clone(&sem)).collect();
    thread::spawn(move || {
        println!("Thread {thread_index} rendezvous");
        let mut current_count = count.lock().unwrap();
        *current_count += 1;
        println!("Thread {thread_index} mutate");
        if *current_count == thread_count {
            barrier.release();
        }
        drop(current_count); // Missing this piece, this was frustrating
        barrier.acquire();
        println!("Thread {thread_index} critical point");
        barrier.release();
    })
}

fn problem_3_6() {
    let thread_count = 4;
    let barrier = Arc::new(Semaphore::new(0));
    let count = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for index in 0..thread_count {
        handles.push(problem_3_6_thread(
            barrier.clone(),
            count.clone(),
            index,
            thread_count,
        ));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
fn problem_3_7_thread(
    turnstile: Arc<Semaphore>,
    turnstile2: Arc<Semaphore>,
    count: Arc<Mutex<i64>>,
    thread_index: i64,
    thread_count: i64,
) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("Thread {thread_index} rendezvous");
        let mut count_guard = count.lock().unwrap();
        *count_guard += 1;
        if *count_guard == thread_count {
            println!("First barrier released");
            turnstile2.acquire();
            turnstile.release();
        }

        turnstile.acquire();
        turnstile.release();
        {
            let mut count_guard = count.lock().unwrap();
            *count_guard -= 1;
            println!("Thread {thread_index} second mutate");
            if *count_guard == 0 {
                println!("Second barrier released");
                turnstile.acquire();
                turnstile2.release();
            }
        }
        turnstile2.acquire();
        turnstile2.release();
        println!("Thread {thread_index} critical point");
    })
}

fn problem_3_7() {
    let thread_count = 4;
    let turnstile0 = Arc::new(Semaphore::new(0));
    let turnstile1 = Arc::new(Semaphore::new(1));
    let count = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for index in 0..thread_count {
        handles.push(problem_3_7_thread(
            turnstile0.clone(),
            turnstile1.clone(),
            count.clone(),
            index,
            thread_count,
        ));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

struct Dancer {
    is_leader: bool,
    id: i64,
}

fn problem_3_8_exclusive_thread(
    leader_semaphore: Arc<Semaphore>,
    internal_leader_semaphore: Arc<Semaphore>,
    follow_semaphore: Arc<Semaphore>,
    internal_follow_semaphore: Arc<Semaphore>,
    current_follower_id: Arc<Mutex<i64>>,
    dancer: Dancer,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let id = dancer.id;
        if dancer.is_leader {
            let mut selected = false;

            while !selected {
                internal_leader_semaphore.acquire();
                follow_semaphore.acquire();
                println!("Leader {id} start");
                {
                    let current_follower_id_value = current_follower_id.lock().unwrap();
                    if *current_follower_id_value == dancer.id {
                        println!("Leader {id} acquired");
                        selected = true;
                    } else {
                        internal_leader_semaphore.release();
                        follow_semaphore.release();
                        thread::sleep(time::Duration::from_millis(10));
                    }
                }
            }
            leader_semaphore.release();
            println!("Leader {id} danced");
            internal_leader_semaphore.release();
        } else {
            internal_follow_semaphore.acquire();
            {
                let mut current_follower_id_value = current_follower_id.lock().unwrap();
                *current_follower_id_value = id;
            }
            println!("Follow {id} locked");
            follow_semaphore.release();
            leader_semaphore.acquire();
            println!("Follower {id} danced");
            internal_follow_semaphore.release();
        }
    })
}

pub fn problem_3_8_exclusive() {
    //Followers will 'wait' for their associated leader
    //Set a mutex with the id of the current follower
    //If a leader aquires the semaphore but their follower isn't there yet, they must relinquish

    let leader_semaphore = Arc::new(Semaphore::new(0));
    let internal_leader_semaphore = Arc::new(Semaphore::new(1));
    let follow_semaphore = Arc::new(Semaphore::new(0));
    let internal_follow_semaphore = Arc::new(Semaphore::new(1));
    let current_follower_id = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for id in 0..4 {
        handles.push(problem_3_8_exclusive_thread(
            leader_semaphore.clone(),
            internal_leader_semaphore.clone(),
            follow_semaphore.clone(),
            internal_follow_semaphore.clone(),
            current_follower_id.clone(),
            Dancer {
                is_leader: true,
                id,
            },
        ));

        handles.push(problem_3_8_exclusive_thread(
            leader_semaphore.clone(),
            internal_leader_semaphore.clone(),
            follow_semaphore.clone(),
            internal_follow_semaphore.clone(),
            current_follower_id.clone(),
            Dancer {
                is_leader: false,
                id,
            },
        ));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
