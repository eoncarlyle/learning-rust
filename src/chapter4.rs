use rand::Rng;

use crate::lbs::Semaphore;
use std::collections::LinkedList;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn problem_4_1_producer(
    queue_semaphore: Arc<Semaphore>,
    buffer_semaphore: Arc<Semaphore>,
    buffer: Arc<Mutex<Vec<String>>>,
) -> JoinHandle<()> {
    return thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|l| l.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            buffer_semaphore.acquire();
            {
                let mut next_buffer = Vec::new();
                http_request.iter().for_each(|line| {
                    next_buffer.push(line.to_string());
                });

                let mut buffer = buffer.lock().unwrap();
                *buffer = next_buffer;
            }
            buffer_semaphore.release();
            queue_semaphore.release();
        }
    });
}

fn problem_4_1_consumer(
    queue_semaphore: Arc<Semaphore>,
    buffer_semaphore: Arc<Semaphore>,
    buffer: Arc<Mutex<Vec<String>>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            queue_semaphore.acquire();
            buffer_semaphore.acquire();

            {
                let mut buffer = buffer.lock().unwrap();
                buffer.iter().for_each(|line| {
                    println!("{}", line);
                })
            }

            buffer_semaphore.release();
        }
    })
}

pub fn problem_4_1() {
    let queue_semaphore = Arc::new(Semaphore::new(0));
    let buffer_semaphore = Arc::new(Semaphore::new(1));
    let buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let producer = problem_4_1_producer(
        queue_semaphore.clone(),
        buffer_semaphore.clone(),
        buffer.clone(),
    );
    let consumer = problem_4_1_consumer(
        queue_semaphore.clone(),
        buffer_semaphore.clone(),
        buffer.clone(),
    );
    producer.join().unwrap();
    consumer.join().unwrap();
}

fn problem_4_1_4_producer(
    queue_semaphore: Arc<Semaphore>,
    capacity_semaphore: Arc<Semaphore>,
    mutex: Arc<Semaphore>,
    buffer: Arc<Mutex<LinkedList<String>>>,
) -> JoinHandle<()> {
    return thread::spawn(move || {
        let mut rng = rand::rng();

        loop {
            let new_value = rng.random_range(0..=10).to_string();
            println!("Produced value: {new_value}");

            capacity_semaphore.acquire();
            mutex.acquire();
            {
                let mut buffer = buffer.lock().unwrap();
                buffer.push_back(new_value);
            }
            mutex.release();
            queue_semaphore.release();
        }
    });
}

fn problem_4_1_4_consumer(
    queue_semaphore: Arc<Semaphore>,
    capacity_semaphore: Arc<Semaphore>,
    mutex: Arc<Semaphore>,
    buffer: Arc<Mutex<LinkedList<String>>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            queue_semaphore.acquire();
            mutex.acquire();

            {
                let mut buffer = buffer.lock().unwrap();
                let maybe_value = buffer.pop_front();

                match maybe_value {
                    Some(value) => {
                        println!("Consumed value: {value}");
                        thread::sleep(Duration::from_secs(10));
                        capacity_semaphore.release();
                    }
                    None => {}
                }
            }

            mutex.release();
        }
    })
}

pub fn problem_4_1_4() {
    let capacity = 3;

    let queue_semaphore = Arc::new(Semaphore::new(0));
    let capacity_semaphore = Arc::new(Semaphore::new(capacity));
    let buffer_semaphore = Arc::new(Semaphore::new(1));
    let buffer: Arc<Mutex<LinkedList<String>>> = Arc::new(Mutex::new(LinkedList::new()));

    let producer = problem_4_1_4_producer(
        queue_semaphore.clone(),
        capacity_semaphore.clone(),
        buffer_semaphore.clone(),
        buffer.clone(),
    );
    let consumer = problem_4_1_4_consumer(
        queue_semaphore.clone(),
        capacity_semaphore.clone(),
        buffer_semaphore.clone(),
        buffer.clone(),
    );
    producer.join().unwrap();
    consumer.join().unwrap();
}
