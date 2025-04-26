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

type Lightswitch() =
    let mutable counter = 0
    let mutex = new Semaphore(1, 1)
    
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
        