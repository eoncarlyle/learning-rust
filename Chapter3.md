## Section 3.3

Puzzle: Generalize the signal pattern so that it works both ways. Thread A has
to wait for Thread B and vice versa. In other words, given this code

```
            Thread A                          Thread B
     +----------------------+          +----------------------+
   1 | statement a1         |        1 | statement b1         |
   2 | statement a2         |        2 | statement b2         |
     +----------------------+          +----------------------+
```

The solution ends up being

```rust
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
```

But the following would have deadlocked

```rust
let handle_a = thread::spawn(move || {
    println!("statement a1");
    sem_a_done_clone_a.acquire();
    sem_b_done_clone_a.release();
    println!("statement a2");
});

let handle_b = thread::spawn(move || {
    println!("statement b1");
    sem_b_done_clone_b.acquire();
    sem_a_done_clone_b.release();
    println!("statement b2");
});
```
