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
