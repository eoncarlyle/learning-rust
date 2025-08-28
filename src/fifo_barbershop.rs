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

    let chair_semaphores: Arc<Vec<_>> = Arc::new(
        (0..CHAIR_COUNT)
            .into_iter()
            .map(|_| Semaphore::new(1))
            .collect(),
    );

    let request_semaphores: Arc<Vec<_>> = Arc::new(
        (0..CHAIR_COUNT)
            .into_iter()
            .map(|_| Semaphore::new(0))
            .collect(),
    );

    let (tx, rx) = channel::<usize>();
    let arctx = Arc::new(tx);

    fn set_chair(chair_scoreboard: &Arc<Vec<AtomicBool>>, chair_idx: usize, value: bool) {
        let chair = chair_scoreboard.get(chair_idx);
        //println!("Load: {chair_idx}/{value}");
        chair.map(|a| {
            a.store(value, SeqCst);
        });
    }

    fn customer(
        label: String,
        scoreboard_mutex: Arc<Semaphore<i32>>,
        chair_scoreboard: Arc<Vec<AtomicBool>>,
        request_semphores: Arc<Vec<Semaphore>>,
        tx: Arc<Sender<usize>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                scoreboard_mutex.acquire();
                let mut selected_idx = CHAIR_COUNT + 1;
                for chair_idx in 0..CHAIR_COUNT {
                    if chair_scoreboard.get(chair_idx).unwrap().load(SeqCst) {
                        selected_idx = chair_idx;
                        set_chair(&chair_scoreboard, selected_idx, false);
                        break;
                    } else {
                    }
                }
                //println!("{}", b);
                scoreboard_mutex.release();
                if selected_idx == CHAIR_COUNT + 1 {
                    //println!("Customer {label} balked");
                    thread::sleep(Duration::from_millis(400));
                } else {
                    println!("Request: {label}/{selected_idx}");
                    tx.send(selected_idx).unwrap();
                    &request_semphores[selected_idx].acquire();
                    //println!("Response: {label}/{selected_idx}");

                    scoreboard_mutex.acquire();
                    set_chair(&chair_scoreboard, selected_idx, true);
                    scoreboard_mutex.release();
                }
                thread::sleep(Duration::from_millis(400));
            }
        })
    }

    fn barber(rx: Receiver<usize>, request_semphores: Arc<Vec<Semaphore>>) -> JoinHandle<()> {
        thread::spawn(move || {
            rx.iter().for_each(|idx| {
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
            request_semaphores.clone(),
            arctx.clone(),
        );
    });

    barber(rx, request_semaphores).join();
}
