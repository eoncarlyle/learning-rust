# README

Learning Rust by working through "The Little Book of Sempahores". Hat tip to Sean Chen for his [semaphore implementation](https://seanchen1991.github.io/posts/sync-primitives-semaphores/).


## Prompt snippets

> Provide minimal code examples: I want to understand this concept, don't hesitate to ask me questions or probe to build my understanding.

> I'd like to work through this problem on my own first. Could you just give me high-level guidance or confirm my understanding without showing specific code implementations? After I've had a chance to solve it myself, I'll share my solution


## `121e445`

### Question
Q)  I am facing

```
error[E0373]: closure may outlive the current function, but it borrows `sem`, which is owned by the current function
```
Does this mean that the compiler cannot prove (nor should it be able to) that Semaphor has a lifetime long enough that it will still exist once that lambda is run and the threads are created?

A) This is correct. The error message "may outlive borrowed value sem" indicates that the Rust compiler can't guarantee that the sem variable will live long enough to be safely used in the closures passed to thread::spawn. The `move` keyword on `sem` would work if there was only one thread. This would tell the compiler to drop `sem` once the stack pointer would leave that closure. This doesn't work for two threads, as there is not compile-time gaurentee about which thread will complete first. One solution is using automic reference counting.

When automic reference counting is used, both threads are given a copy of a pointer to the semaphor object in memory. The lifetime of both ARC pointers is evaluated by the compiler, and the ARC count is decremented once the lifetime is over. At runtime, once the reference count is zero the original sempahore is dropped.

### Commentary
> Arc<T> is Rust's ambassador.
> It can share values across threads, guaranteeing that these will not interfere with each other.

`Arc<T>` poitners are intialised like

```rust
let a = Arc::new(String::from("arc reference!"));

let a_clone_0 = Arc::clone(&a);
let a_clone_1 = Arc::clone(&a)w
```

Where `a_clone_0` and `a_clone_1` can have seprately evaluated lifetimes.

## `7b56224`
Q) On build I see
```
error[E0596]: cannot borrow data in an `Arc` as mutable
```
 This isn't an issue of lifetimes and memory allocations, but rather an issue of borrow checking and memory safety. My interpretation of the compiler is that it isn't allowing a mutable reference to count. Sure, I might not be modifying the memory that goes into that pointer from that code, but the compiler was designed with rigorous guarantees in mind.

A) `Arc` is specifically designed for shared ownership across threads with immutable access. Calling `get_mut()` on `Arc`  is an attempt to get exclusive mutable access to the contained memory. This would be fine if there is only one reference to that pointer, but by cloning the `Arc` the compiler 'knows' that this is not the case.

I know that this is a little strange to be using mutexes in an exercise where you're trying to use _semaphores_ as mutexes, but because there wasn't some special justification for using the atomic integer (and because we're only using this mutex to be able to print out the result at the end of the exercise), let's try out an `Arc<Mutex<T>>`
