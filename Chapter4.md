# Chapter 4

## Section 4.1
Multithreaded programs with division of labour between threads require that a single thread has exclusive access to the shared buffer between consumers and producers. Furthermore, consumer threads must block on an empty buffer.

## 4.1.2: Producer-consumer
```typescript
//Producer
event = waitForEvent()
mutex.wait()
buffer.add(event)

mutex.signal()
items.signal()
```

```typescript
//Consumer
mutex.wait()
items.wait()
event = buffer.get()

mutex.signal()
event.process()
```

The above pair of consumers and producers deadlocks. If the consumer aquires the mutex semaphore first before waiting on the items semaphore, the producer is blocked resulting in a deadlock.

Lesson: aany time you wait for a semaphore while holding a mutex, there is danger of a deadlock.

# 4.2: Readers-writers problem

When attempting to solve this one, I was not able to think of a way to lock out only writer threads and no reader threads when one reader has started. I think the path forward is to have a mutex set the state of the threads - reading or writing
