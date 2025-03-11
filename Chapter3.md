## Section 3.3

Puzzle: Generalize the signal pattern so that it works both ways. Thread A has
to wait for Thread B and vice versa. In other words, given this code

```text
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

## Section 3.6


Puzzle: Generalize the rendezvous solution. Every thread should run the
following code:

```text
            Barrier code
     +----------------------+
   1 | rendezvous            |
   2 | critical point        |
     +----------------------+
```

The synchronization requirement is that no thread executes critical point
until after all threads have executed rendezvous.

Solution ends up being

```text
rendevous
mutex.wait()
  count = count + 1
mutex.signal()

if count == n: barrier.signal()

barrrier.wait()
barrier.signal()
critical point
```

Wait and signal in a rapid signal is a 'turnstile' and requires one thread to pass at a time and can be barred to prevent any threads from passing

## Section 3.7

The naive solution is

```text
rendevous
mutex.wait()
  count += 1
mutex.signal()
if count == n: turnstile.signal()

turnstile.wait()
turnstile.signal()

critical point
mutex.wait()
  count -= 1
mutex.signal()
if count == 0: turnstile.wait()
```
However, if the _n_ - 1th thread is interupted when the turstile is first signaled, then multiple threads may signal the turnstile.

A further iteration is to move the turnstiles inside of the mutexes. Now a thread cannot be interpupted after changing the counter

```text
rendevous
mutex.wait()
  count += 1
  if count == n: turnstile.signal()
mutex.signal()

turnstile.wait()
turnstile.signal()

critical point
mutex.wait()
  count -= 1
  if count == 0: turnstile.wait()
mutex.signal()
```

The problem is that a single thread can pass through the second mutex and then be put back in place into the second.

My attempt:
```
turnstile0.wait()
turnstile0.signal()
rendevous
mutex.wait()
  count += 1
  if count == n: turnstile1.signal()
mutex.signal()

turnstile1.wait()
turnstile1.signal()

critical point
mutex.wait()
  count -= 1
  if count == 0: turnstile0.wait()
mutex.signal()
```

```
rendevous
mutex.wait()
  count += 1
  if count == n:
    turnstile1.wait()
    turnstile0.signal()
mutex.signal()

turnstile1.wait()
turnstile1.signal()

critical point
mutex.wait()
  count -= 1
  if count == 0:
    turnstile0.wait()
    turnstile2.signal()
mutex.signal()
turnstile2.wait()
turnstile2.signal()
```
