# README

Learning Rust by working through ["The Little Book of Semaphores"](https://greenteapress.com/wp/semaphores/). Hat tip to Sean Chen for his [semaphore implementation](https://seanchen1991.github.io/posts/sync-primitives-semaphores/).


## Prompt snippets

Learning Approach: I'm doing deliberate practice on concurrency concepts. Please:
- Ask probing questions to test my understanding rather than just giving answers
- Let me work through problems step-by-step before providing solutions
- Point out when I'm on the right track vs. when I need to reconsider
- Help me connect specific solutions back to general principles
- Challenge me to explain WHY things work, not just HOW"

Feedback Preferences: when I share code or ideas:
- Identify gaps in my reasoning without immediately filling them
- Ask 'What do you think happens if...' style questions
- Help me discover issues through guided questioning
- Only show code examples after I've attempted the concept myself"

Learning Goals:
- I want to build transferable understanding of synchronization primitives, not just memorize patterns
- Help me see the deeper principles behind semaphores, mutexes, and condition variables that apply across different problems

Engagement Level:
- Feel free to push back on my assumptions and make me justify my reasoning
- I learn better when I have to defend and refine my thinking.

## `121e445`

### Question

```
error[E0373]: closure may outlive the current function, but it borrows `sem`, which is owned by the current function
```
Does this mean that the compiler cannot prove (nor should it be able to) that the semaphore has a lifetime long enough that it will still exist once that lambda is run and the threads are created?

### Answer
This is correct. The error message "may outlive borrowed value sem" indicates that the Rust compiler can't guarantee that the sem variable will live long enough to be safely used in the closures passed to thread::spawn. The `move` keyword on `sem` would work if there was only one thread. This would tell the compiler to drop `sem` once the stack pointer would leave that closure. This doesn't work for two threads, as there is not compile-time guarantee about which thread will complete first. One solution is using atomic reference counting.

When atomic reference counting is used, both threads are given a copy of a pointer to the semaphore object in memory. The lifetime of both ARC pointers is evaluated by the compiler, and the ARC count is decremented once the lifetime is over. At runtime, once the reference count is zero the original semaphore is dropped.

### Commentary
> Arc<T> is Rust's ambassador.
> It can share values across threads, guaranteeing that these will not interfere with each other.

`Arc<T>` pointers are initialised like

```rust
let a = Arc::new(String::from("arc reference!"));

let a_clone_0 = Arc::clone(&a);
let a_clone_1 = Arc::clone(&a);
```

Where `a_clone_0` and `a_clone_1` can have separately evaluated lifetimes.

## `7b56224`

### Question
On build I see
```
error[E0596]: cannot borrow data in an `Arc` as mutable
```
 This isn't an issue of lifetimes and memory allocations, but rather an issue of borrow checking and memory safety. My interpretation of the compiler is that it isn't allowing a mutable reference to count. Sure, I might not be modifying the memory that goes into that pointer from that code, but the compiler was designed with rigorous guarantees in mind.

### Answer
`Arc` is specifically designed for shared ownership across threads with immutable access. Calling `get_mut()` on `Arc`  is an attempt to get exclusive mutable access to the contained memory. This would be fine if there is only one reference to that pointer, but by cloning the `Arc` the compiler 'knows' that this is not the case.

I know that this is a little strange to be using mutexes in an exercise where you're trying to use _semaphores_ as mutexes, but because there wasn't some special justification for using the atomic integer (and because we're only using this mutex to be able to print out the result at the end of the exercise), let's try out an `Arc<Mutex<T>>`

## `57eca3a`

### Question
Across two threads I am locking the same mutex without ever explicitly unlocking it. While I am surprised that this even built - let alone that it worked - my _guess_ is that any locks made by an `Arc` clone are discarded when the arc clone is dropped. I'm not 100% sure about this because that would seem to require some explicit relationship between `Arc` and `Mutex` that I'm not sure about. Why didn't I have to unlock?

### Answer
Rust's  `Mutex` uses 'RAII (Resource Acquisition Is Initialization)'. The return type of `count_clone_a.lock().unwrap` is a `MutexGuard<T>`. This both provides access to the data and then releases the lock when it goes out of scope. This is not the last that you will see of RAII in Rust: tying resource management to variable scope is used in file handles, network connections, and other similar resources

## `db9f845`
### Question
When using `handles.iter().for_each(|handle| handle.join().unwrap());` in place of the for loop, the build error
```
rustc: cannot move out of *handle which is behind a shared reference
```
was provided. Why is the iterator different than the for loop? I would have thought the ownership was clear?

### Answer
The problem is that `handles.iter()` provides shared `&JoinHandle<()>` references to the handles but does not grant ownership of them.
1) `iter()` iterates over references `&T` rather than the values themselves
2) `for_each` tries to call `join()` on the references
3) `join()` requires ownership of `JoinHandle` rather than `&JoinHandle`

Either `into_iter` or for loop forgoes this problem. When `into_iter()` is called, the iterator becomes the owner of all elements.

## `afb0f86`
### Question
I need a mutex that I can lock and unlock on demand, but the Rust lifetime model makes it hard to do that. I didn't have this issue on a similar, single-phase barrier. Is there a concurrency primitive that I do not know about, or are there questions I need to ask myself about mutexes?

### Answer
For the mutex issue, one approach is to ensure you're using very small, well-defined scopes for each mutex operation. By enclosing each reference to the mutex in curly braces, two seprate references can be made to the lock:

```rust
fn problem_3_7_thread(
    turnstile: Arc<Semaphore>,
    turnstile2: Arc<Semaphore>,
    count: Arc<Mutex<i64>>,
    thread_index: i64,
    thread_count: i64,
) -> JoinHandle<()> {
    return thread::spawn(move || {
        println!("Thread {thread_index} rendezvous");
        {
            let mut count_guard = count.lock().unwrap();
            *count_guard += 1;
            if *count_guard == thread_count {
                println!("First barrier released");
                turnstile2.acquire();
                turnstile.release();
            }
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
    });
}
```

## `11b127e`
### Question
While I have used atomic reference counting to share things like shared semaphores, this is the first time that I've had a mutable list - needed for the queue to work - that I need to share between both threads. This is exactly the type of thing that Rust was designed to prevent issues with. Before I use the 'unsafe' keyword, are there some Rust smart pointers that would help me safely handle this list?

### Answer
The `Arc<LinkedList<String>>` only provides shared immutable access, wheras mutable access is needed to add and remove from the queue. The most idiomatic way to accomplish this is using a mutex with an Arc.

## `74b6cd6`
### Question
Unfortunately the book was pretty vague about the format of the solution - it's not like they said 'conform to this interface. I think this might be a situation where your \[Claude's\] solution is more correct than the provided solution. As a large langauge model you might already know this, but I am using this book to learn Rust (and semaphores).

My Rust 'solution' (that doesn't build) faces a problem where if I need to share a mutable queue across threads, I run into ownership problems. There seems to be a deeper lesson here. By moving from an explicit queue to an implicit queue (calling the same function across different dancer inputs) there aren't mutable references that need to be handed back and forth. Are there other lessons here?

### Answer
The book doesn't give a lot of detail but it more or less uses the semaphores as an implicit queue. Just like was done for other problems in the book, it is still the right choice to use seprate threads for each dancer. Two functions, `leader` and `follower`, should be delcared with two `Arc<Semaphore>` arguments. Then it is only a matter of cloning semaphores, spawning threads, and moving ownership in exactly the same way that has been done in the rest of the book.
