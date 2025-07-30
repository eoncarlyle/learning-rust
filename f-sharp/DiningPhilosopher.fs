module DiningPhilosopher

open System.Threading.Tasks
open System.Threading
open System
open Common


let left i = i
let right i = (i + 1) % 5
let forks = [ for i in 0..5 -> new Semaphore(1, 1) ]

let philosopher (i: int) =
    task {
        do! Task.Delay(750)
        Console.WriteLine $"{i} done"
    }

let problem_4_4_3 () =
    let occupancy = Semaphore(4, 4)

    let cappedPhilosphier (i: int) =
        task {
            do! Task.Delay(750)
            toggleSem occupancy Wait
            let leftFork = forks.Item(left i)
            let rightFork = forks.Item(right i)

            toggleSem leftFork Wait
            toggleSem rightFork Wait
            Console.WriteLine $"{i} eating"
            do! Task.Delay(750)
            Console.WriteLine $"{i} done"

            toggleSem leftFork Release
            toggleSem rightFork Release
            toggleSem occupancy Release
        }


    Task.WhenAll [ for i in 0..5 -> cappedPhilosphier (i) ]

let main () = problem_4_4_3 () |> _.Result
