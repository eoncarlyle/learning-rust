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
