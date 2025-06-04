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
