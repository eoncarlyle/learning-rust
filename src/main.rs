use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use lbs::Semaphore;
mod lbs;

fn main() {
    problem_3_6();
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
    let sem_b_done = Arc::new(lbs::Semaphore::new(0));
    let sem_a_done = Arc::new(lbs::Semaphore::new(0));

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
    let mutex = Arc::new(lbs::Semaphore::new(1));
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

fn problem_3_6_section(
    barrier: Arc<Semaphore>,
    count: Arc<Mutex<i64>>,
    thread_index: i64,
    thread_count: i64,
) -> JoinHandle<()> {
    //let sem_clones: Vec<Arc<Semaphore>> = sems.iter().map(|sem| Arc::clone(&sem)).collect();
    return thread::spawn(move || {
        println!("Thread {thread_index} rendezvous");
        let mut current_count = count.lock().unwrap();
        *current_count += 1;
        if *current_count == thread_count {
            barrier.release();
        }
        barrier.acquire();
        println!("Thread {thread_index} critical point");
        barrier.release();
    });
}

fn problem_3_6() {
    let thread_count = 4;
    let barrier = Arc::new(lbs::Semaphore::new(0));
    let count = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for index in 0..thread_count {
        handles.push(problem_3_6_section(
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
