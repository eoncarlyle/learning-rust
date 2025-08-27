use crate::lbs::Semaphore;
use std::sync::Arc;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn run() {
    static CHAIR_COUNT: usize = 5;

    let chair_scoreboard: Arc<Vec<AtomicBool>> = Arc::new(
        (0..CHAIR_COUNT)
            .into_iter()
            .map(|_| AtomicBool::new(true))
            .collect(),
    );

    let scoreboard_mutex = Arc::new(Semaphore::new(1));

    let chair_semaphores: Arc<Vec<_>> = Arc::new(
        (0..CHAIR_COUNT)
            .into_iter()
            .map(|_| Semaphore::new(1))
            .collect(),
    );

    let (tx, rx) = channel::<usize>();
    let arctx = Arc::new(tx);

    // The
    fn set_chair(chair_scoreboard: &Arc<Vec<AtomicBool>>, chair_idx: usize, value: bool) {
        println!("Load: {chair_idx}/{value}");
        let chair = chair_scoreboard.get(chair_idx);
        chair.map(|a| {
            if a.load(SeqCst) {
                a.store(value, SeqCst);
            }
        });
    }

    fn customer(
        label: String,
        scoreboard_mutex: Arc<Semaphore<i32>>,
        chair_scoreboard: Arc<Vec<AtomicBool>>,
        chair_semaphores: Arc<Vec<Semaphore>>,
        tx: Arc<Sender<usize>>,
    ) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                scoreboard_mutex.acquire();
                let mut selected_idx = CHAIR_COUNT + 1;
                for chair_idx in 0..CHAIR_COUNT {
                    //let a = chair_scoreboard.get(chair_idx).unwrap();
                    //let b = if a.load(SeqCst) { 1 } else { 0 };
                    //println!("{b}");
                    if chair_scoreboard.get(chair_idx).unwrap().load(SeqCst) {
                        selected_idx = chair_idx;
                        println!("Tjread {label}");
                        set_chair(&chair_scoreboard, chair_idx, false);
                        continue; //bug!
                    }
                }
                scoreboard_mutex.release();
                println!("{label}, {selected_idx}");
                if selected_idx == CHAIR_COUNT + 1 {
                    //println!("Customer {} balked", label);
                    thread::sleep(Duration::from_millis(100));
                } else {
                    tx.send(selected_idx);
                    &chair_semaphores[selected_idx].acquire();

                    scoreboard_mutex.acquire();
                    set_chair(&chair_scoreboard, selected_idx, true);
                    println!("{label}");
                    scoreboard_mutex.release();
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    fn barber(rx: Receiver<usize>, chair_semaphores: Arc<Vec<Semaphore>>) -> JoinHandle<()> {
        return thread::spawn(move || {
            loop {
                let idx = rx.recv().unwrap();

                //Ideally I'd do this more monadically, but we are going from result to option
                let sem = &chair_semaphores[idx];
                println!("Barber cutting {idx}");
                sem.release();
                thread::sleep(Duration::from_millis(400));
            }
        });
    }

    (0..10).for_each(|l| {
        customer(
            l.to_string(),
            scoreboard_mutex.clone(),
            chair_scoreboard.clone(),
            chair_semaphores.clone(),
            arctx.clone(),
        );
    });

    barber(rx, chair_semaphores).join();
}
