use rand::Rng;

use crate::lbs::Semaphore;
use std::collections::LinkedList;
use std::env::VarsOs;
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

#[derive(Clone)]
enum ReaderWriterIoState {
    Reading,
    Writing,
}

struct ReaderWriterState {
    value: i64,
    io_state: ReaderWriterIoState,
    reader_count: i64,
    writer_count: i64,
}

fn problem_4_2_reader(
    mutex_sem: Arc<Semaphore>,
    state: Arc<Mutex<ReaderWriterState>>,
    label: String,
) {
    thread::spawn(move || {
        loop {
            mutex_sem.acquire();
            let mut thread_state = state.lock().unwrap();
            let io_state = thread_state.io_state.clone();
            let reader_count = thread_state.reader_count;
            let writer_count = thread_state.writer_count;

            match (io_state, reader_count, writer_count) {
                (ReaderWriterIoState::Reading, readers, 0) if readers >= 0 => {
                    thread_state.reader_count += 1;
                    let read_value = thread_state.value;
                    mutex_sem.release();
                    drop(thread_state);
                    println!("R{label}: Read {read_value}");
                    mutex_sem.acquire();
                    let mut inner_state = state.lock().unwrap();
                    inner_state.reader_count -= 1;
                    drop(inner_state);
                    mutex_sem.release();
                }
                (ReaderWriterIoState::Writing, 0, 0) => {
                    thread_state.io_state = ReaderWriterIoState::Reading;

                    thread_state.reader_count += 1;
                    let read_value = thread_state.value;
                    mutex_sem.release();
                    drop(thread_state);
                    println!("R{label}: Read {read_value}");
                    mutex_sem.acquire();
                    let mut inner_state = state.lock().unwrap();
                    inner_state.reader_count -= 1;
                    drop(inner_state);
                    mutex_sem.release();
                }
                (ReaderWriterIoState::Writing, 0, writers) if writers >= 0 => {
                    println!("$R{label}: waiting");
                    mutex_sem.release();
                }
                _ => {
                    //let disp = match io_state {
                    //    ReaderWriterIoState::Writing => "write",
                    //    ReaderWriterIoState::Reading => "read",
                    //};
                    let disp = "todo";
                    panic!("Illegal state: {disp}, {reader_count}, {writer_count}");
                }
            }

            println!("Writer thread critical: {label}");
            mutex_sem.release();
            println!("Writer thread release: {label}");
        }
    });
}

fn problem_4_2_writer(
    mutex_sem: Arc<Semaphore>,
    state: Arc<Mutex<ReaderWriterState>>,
    label: String,
) {
    thread::spawn(move || {
        loop {
            mutex_sem.acquire();
            let mut thread_state = state.lock().unwrap();
            let io_state = thread_state.io_state.clone();
            let reader_count = thread_state.reader_count;
            let writer_count = thread_state.writer_count;

            match (io_state, reader_count, writer_count) {
                (ReaderWriterIoState::Writing, 0, writing) if writing >= 0 => {
                    thread_state.reader_count += 1;
                    let read_value = thread_state.value;
                    mutex_sem.release();
                    drop(thread_state);
                    println!("R{label}: Read {read_value}");
                    mutex_sem.acquire();
                    let mut inner_state = state.lock().unwrap();
                    inner_state.reader_count -= 1;
                    drop(inner_state);
                    mutex_sem.release();
                }
                (ReaderWriterIoState::Reading, 0, 0) => {
                    thread_state.io_state = ReaderWriterIoState::Writing;

                    thread_state.reader_count += 1;
                    let read_value = thread_state.value;
                    mutex_sem.release();
                    drop(thread_state);
                    println!("R{label}: Read {read_value}");
                    mutex_sem.acquire();
                    let mut inner_state = state.lock().unwrap();
                    inner_state.reader_count -= 1;
                    drop(inner_state);
                    mutex_sem.release();
                }
                (ReaderWriterIoState::Writing, 0, writers) if writers >= 0 => {
                    println!("$R{label}: waiting");
                    mutex_sem.release();
                }
                _ => {
                    //let disp = match io_state {
                    //    ReaderWriterIoState::Writing => "write",
                    //    ReaderWriterIoState::Reading => "read",
                    //};
                    let disp = "todo";
                    panic!("Illegal state: {disp}, {reader_count}, {writer_count}");
                }
            }

            state.lock().unwrap();
            println!("Writer thread critical: {label}");
            mutex_sem.release();
            println!("Writer thread release: {label}");
        }
    });
}

pub fn problem_4_2() {}
