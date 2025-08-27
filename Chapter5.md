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
