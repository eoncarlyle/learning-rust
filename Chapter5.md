# Capter 5

## Communal Pot Problem
Possible pitfalls
- Diners attempt to take a serving before the cook has refilled: diners should not attempt to get a serving unless the cook has refilled the pot
- I am thinking that Downey's introduction of a scoreboard is relevant here
- The cook should be woken up directly on the

- Intitial quasi-solution alerted the cook multiple times which broke the problem invariants

## Communal Pot Attempt Comentary
> A \[group of diners\] eats communal dinners from a large pot ... When a \[diner\] wants to
eat, he helps himself from the pot, unless it is empty. If the pot is
empty...wakes up the cook and then waits until the cook
has refilled the pot.

My solution did not satisfy this because the diner that wakes the cook is not gaurenteed to be the first diner to be served. Also, I had to inelegantly resort to a magic counter value and I'd prefer to not do that. The way around this is for request/response semaphores to be used. This is not actually a design pattern laid out in the book, but it seems mmeaningful

```typescript
//requester
sempahoreA.signal()
semaphoreB.wait()
```

```typescript
//responder
semaphoreA.wait()
// ...
semaphoreB.signal()
```

"The elegance comes from separating "pot is empty" (resource exhausted) from "pot is full" (resource replenished) into distinct channels. Many bugs come from trying to overload one signaling mechanism."

"The core principle: When you need deterministic ordering or specific thread selection, basic semaphores aren't enough because their wakeup order is implementation-dependent. You need explicit coordination mechanisms."

## FIFO Barbershop
- My first thought was to make a list of Semaphores for the available seats, and use `std::sync::mpsc::channel` to queue indicies of customers as they sit down
- However, doing this without introducing a race condition in between checking the scoreboard and trying to sit down could get annoying
- We need to have specific sempahores for each seat to make this work, but I think placing the semaphore into the queue is the best way to handle this
- If I wanted to stick with atomic references, the scoreboard could be the lowest free index (Sempahore + atomic int like before)
- I think overall I am making this too complicated

"The type you're seeing &<Vec<Semaphore> as Index<i32>>::Output is Rust's way of showing that it's trying to use i32 as an index, but Vec<T> doesn't implement Index<i32> - only Index<usize>." This was the issue with `let (tx, rx) = channel::<i32>();`

- The difference between `fn set_chair(chair_scoreboard: Arc<Vec<AtomicBool>>, chair_idx: usize, value: bool)` and the `&Arc` equivalent are important
- Continue vs. break bug in customer for `6243563`
- Doesn't work!
```rust
        chair.map(|a| {
            if a.load(SeqCst) { //Bug!
                a.store(value, SeqCst);
            }
        });
```
- We have duplicated open chairs because the sempahore is both how the consumers are being seated and how the barber signals - because they are initially open the first request sails through and the second one waits
- Need different seat, request semaphores
- Takeaway: if you are using channels or two other sources of truth, consider a state diagram to keep consumers on the same page
- 'rx.iter() naturally handles the Result → Option conversion and provides a clean functional interface'

### `5f6f558`
- This was broken in two ways
  1) The `tx.send(selected_idx).unwrap()` being outside of the `scoreboard_mutex` meant that the FIFO invariant could be broken by a 'malicous scheduler': no mutex means no ordering gaurentee
  2) It would be possible for the `request_semphores[idx].release();` in the barber to be called before the customer actually signals on the mutex. While this is not per se a requirement, it is definitely in the spirit of the problem
  3) You can end up with a sitation where consumer A reserves a chair before consumer B, but consumer B carries out `customer_sempahores[idx].acquire();` first (this is broken in current solution)
- The greater lesson is
  1) If something needs to be inside of a mutex to be correct without introducing race conditions, you are asking for problems if another thread needs it and you don't keep the _communication_ in the mutex too
  2) I think I have this written down, but the 'cross thread turnstile' that uses a turnstile mediated by two semaphores has a multi-thread analouge and it is needed if you need to gaurentee that a response takes place before a request
  3) If you need strict ordering when sending semaphroes through a queue, you are almost certainly better off sending the actual sempahores rather than indicies to semaphores: if you have one consumer then the consumer should take a 'ready' sempahore and signal it, otherwise multiple publishers can get out of order!
