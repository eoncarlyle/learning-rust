# Chapter 4

## Section 4.1

Multithreaded programs with division of labour between threads require that a single thread has exclusive access to the shared buffer between consumers and producers. Furthermore, consumer threads must block on an empty buffer.

## 4.1.2: Producer-consumer

```typescript
//Producer
event = waitForEvent();
mutex.wait();
buffer.add(event);

mutex.signal();
items.signal();
```

```typescript
//Consumer
mutex.wait();
items.wait();
event = buffer.get();

mutex.signal();
event.process();
```

The above pair of consumers and producers deadlocks. If the consumer aquires the mutex semaphore first before waiting on the items semaphore, the producer is blocked resulting in a deadlock.

Lesson: any time you wait for a semaphore while holding a mutex, there is danger of a deadlock.

## 4.2: Readers-writers problem

### 4.2.2

When attempting to solve this one, I was not able to think of a way to lock out only writer threads and no reader threads when one reader has started. I think the path forward is to have a mutex set the state of the threads - reading or writing.

My solution required a lock over a `ReaderWriterState` that kept track of if the state was reading or writing as well as the counts of readers and writers. The actual solution was something much simpler

```text
int readers = 0
mutex = Semaphore(1)
roomEmpty = Semaphore(1)
```

Where the mutex gaurded the readers count. When the readers dropped to zero they would release the `roomEmpty` mutex. My understanding was not realising how simple the solution could be. Things are not neccesarily made easier by Rust - the examples assume being able to change `readers` directly, something that probably has to be done in an `unsafe` block - but that wasn't the big thing.

### 4.2.3

While the provided solution was

```python
#Writer
turnstile.wait()
    roomEmpty.wait()
    # Critical Section
turnstile.signal()
roomEmpty.signal()
```

```python
#Reader
turnstile.wait()
turnstile.signal()
readSwitch.lock(roomEmpty)
    # Critical section
readSwitch.unlock(roomEmpty)
```

My solution, in the same langauge, was

```python
#Writer
turnstile.wait()
    roomEmpty.wait()
    turnstile.signal()
    # Critical Section
roomEmpty.signal()
```

```python
#Reader
turnstile.wait()
readSwitch.lock(roomEmpty)
turnstile.signal()
    # Critical section
readSwitch.unlock(roomEmpty)
```

I _think_ that this is fine from walking through the progression but for the next problem I will use the author's pattern

### 4.2.6

Solving "allow all threads of one type through first" luckily is something that we have already addressed. If we tie the current turnstile to a lightswitch this should work. We can think of this as if there is a 'chamber' before the critical section

I think the only meaningful difference between my solution and the provided solution is that I do a different ordering for semaphore to lightswitch. For the writer I lock the switch and then wait for the room to be unoccupied and for the writer I wait for the switch-bearing lock before locking a lightswitch. I think that it doens't matter because in the provided solution you have the readers waiting on a lock that the writers lock first, and the other way around in my solution i.e. I think that the following would be a problem

```python
noReaders.wait()
    readSwitch.lock(noWriters)
###
noWriters.wait()
    readSwitch.lock(noReaders)
```

# 4.3 No-starve Mutex
Catagorical starvation: one _kind_ of thread can be blocked forever without intervention. This is different from 'normal' thread starvation where any given thread could wait indefinitely while others proceed. If starvation is unacceptable, **bounded waiting** is a restriction that the amount of time spent waiting is provably fininte. Note that if a thread is never scheduled by the scheduler then it will always starve no matter what we do, so some of the responsibilty rests on the OS scheduler.

The assumption "if there are threads waiting on a semaphore when a thread executes signal, then one of the waiting threads has to be woken." certainly helps, but three threads are running the code below there is no gaurentee that we don't end up in a situation with two threads switching off indefinitely, starving the third.

```python
while True:
    mutex.wait()
    # Critical section
    mutex.signal()
```

Dijkstra came to the conclusion that the number of threads woken by a semphore must be bounded in order to prevent mutex thread starvation, but J.M. Morris used two Semaphore barriers to demonstrate that this wasn't required.

# 4.5 Cigarette Smokers Problem
> The agent repeatedly chooses two diﬀerent ingredients at random and makes
them available to the smokers. Depending on which ingredients are chosen, the
smoker with the complementary ingredient should pick up both resources and
proceed.

The three ingredients are tobacco, paper, and a match. If tobacco and match are selected first (and thus those semaphores are released/signaled), then _ideally_ the smoker with paper will acquire both sempahores and proceed. But if the other two smokers who need matches and tobacco acquire the sempahres first, then both will deadlock on paper.

```text
Smoker with matches
┌─────────────────────┐
│ tobacco.wait()      │
│ paper.wait()        │
│ agentSem.signal()   │
└─────────────────────┘

Smoker with tobacco
┌─────────────────────┐
│ paper.wait()        │
│ match.wait()        │
│ agentSem.signal()   │
└─────────────────────┘

Smoker with paper
┌─────────────────────┐
│ tobacco.wait()      │
│ match.wait()        │
│ agentSem.signal()   │
└─────────────────────┘
```
