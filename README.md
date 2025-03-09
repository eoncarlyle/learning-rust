# README

## Prompt snippets

> Provide minimal code examples: I want to understand this concept, don't hesitate to ask me questions or probe to build my understanding.


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
``

Where `a_clone_0` and `a_clone_1` can have seprately evaluated lifetimes.
