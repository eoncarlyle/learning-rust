module Common
open System.Threading


type ConcurrencyType =
    | ThreadModel
    | TaskModel

type SemaphoreOperation =
    | Release
    | Wait

let toggleSem (sem: Semaphore) (semaphoreOperation: SemaphoreOperation) =
    match semaphoreOperation with
    | Release -> sem.Release() |> ignore
    | Wait -> sem.WaitOne() |> ignore

type Lightswitch(counter, mutex) =
    let mutable counter = counter
    let mutex = mutex
    
    member this.Lock semaphore =
        toggleSem mutex Wait
        counter <- counter + 1
        if counter = 1 then toggleSem semaphore Wait
        toggleSem mutex Release
        
    member this.Unlock semaphore =
        toggleSem mutex Wait
        counter <- counter - 1
        if counter = 0 then toggleSem semaphore Release
        toggleSem mutex Release
        