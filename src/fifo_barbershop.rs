use crate::lbs::Semaphore;
use std::process::id;
use std::sync::Arc;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{thread, vec};

pub fn run() {
    static CHAIR_COUNT: usize = 5;

    let chair_scoreboard: Arc<Vec<AtomicBool>> = Arc::new(
        (0..CHAIR_COUNT)
            .into_iter()
            .map(|_| AtomicBool::new(true))
            .collect(),
    );

    let scoreboard_mutex = Arc::new(Semaphore::new(1));

    let customer_semaphores: Arc<Vec<_>> = Arc::new(
        (0..CHAIR_COUNT)
            .into_iter()
            .map(|_| Semaphore::new(0))
            .collect(),
    );

    let request_mutex = Arc::new(Semaphore::new(0));

    let (tx, rx) = channel::<usize>();
    let arctx = Arc::new(tx);

    fn set_chair(chair_scoreboard: &Arc<Vec<AtomicBool>>, chair_idx: usize, value: bool) {
        let chair = chair_scoreboard.get(chair_idx);
        //println!("Load: {chair_idx}/{value}");
        chair.inspect(|a| {
            a.store(value, SeqCst);
        });
    }

    fn customer(
        label: String,
        scoreboard_mutex: Arc<Semaphore<i32>>,
        chair_scoreboard: Arc<Vec<AtomicBool>>,
        customer_sempahores: Arc<Vec<Semaphore>>,
        request_mutex: Arc<Semaphore<i32>>,
        tx: Arc<Sender<usize>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                scoreboard_mutex.acquire();

                let maybe_selected_index = (0..CHAIR_COUNT)
                    .find(|chair_idx| chair_scoreboard.get(*chair_idx).unwrap().load(SeqCst));

                maybe_selected_index.inspect(|idx| {
                    set_chair(&chair_scoreboard, *idx, false);
                    println!("Request: {label}/{idx}");
                    tx.send(*idx).unwrap();
                });

                scoreboard_mutex.release();

                match maybe_selected_index {
                    None => {
                        //println!("Customer {label} balked");
                    }
                    Some(idx) => {
                        request_mutex.release();
                        customer_sempahores[idx].acquire();
                        scoreboard_mutex.acquire();
                        set_chair(&chair_scoreboard, idx, true);
                        scoreboard_mutex.release();
                    }
                }

                thread::sleep(Duration::from_millis(400));
            }
        })
    }

    fn barber(
        rx: Receiver<usize>,
        request_semphores: Arc<Vec<Semaphore>>,
        request_mutex: Arc<Semaphore<i32>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            rx.iter().for_each(|idx| {
                request_mutex.acquire();
                request_semphores[idx].release();
                println!("Barber cutting {idx}");
                thread::sleep(Duration::from_millis(400));
            });
        })
    }

    (0..10).for_each(|l| {
        customer(
            l.to_string(),
            scoreboard_mutex.clone(),
            chair_scoreboard.clone(),
            customer_semaphores.clone(),
            request_mutex.clone(),
            arctx.clone(),
        );
    });

    barber(rx, customer_semaphores, request_mutex).join();
}
