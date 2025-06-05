module DiningPhilosopher

open System.Threading.Tasks
open System.Threading
open System


let left i = i
let right i = (i + 1) % 5
let forks = [ for i in 0..5 -> new Semaphore(1, 1) ]

let philosopher (i: int) =
    task {
        do! Task.Delay(1000)
        Console.WriteLine $"{i} done"
    }

let tableOccupancy () =
    let cappedOccupancy (i: int) =
        task {
            for i in 1..4 do
                do! Task.Delay(750)
                let leftFork = forks.Item(left i)
                let rightFork = forks.Item(right i)
                Console.WriteLine $"{i} eating"
                do! Task.Delay(750)
        }

    let occupancy = Semaphore(4, 4)

    let philosophers = [ for i in 0..5 -> philosopher (i) ]


    Task.WhenAll philosophers


let main () = philosopher 1 |> Task.WaitAll
