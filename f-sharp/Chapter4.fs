module Chapter4
open System.Runtime
open Common
open System.Threading
open System
open Microsoft.FSharp.Core

// Writers: if the room isn't occupied, toggle the occupancy sempahore
// Readers: keep track of the number readers, if zero then toggle off occupancy

let problem_4_2_write_thread
    (label: int)
    (occupied: Semaphore)
    =
    Thread(fun () ->
        while true do
            Console.WriteLine $"{label} writer waiting"
            toggleSem occupied Wait
            Console.WriteLine $"{label} writer writing"
            Thread.Sleep(300)
            toggleSem occupied Release)

let problem_4_2_read_thread
    (label: int)
    (occupied: Semaphore)
    (lightswitch: Lightswitch)
    =
    Thread(fun () ->
        while true do
            lightswitch.Lock occupied
            Console.WriteLine $"{label} reader reading"
            Thread.Sleep(300)
            //Dropping the parenthesis off of the statement below
            //F# parser limitations
            lightswitch.Unlock occupied)
    
let problem_4_2 =
    let reader_count = 2
    let writer_count = 2
    let occupied = new Semaphore(1, 1)
    let lightswitch = Lightswitch()
   
    let writers = [ 0..writer_count-1 ] |> List.map (fun i -> problem_4_2_write_thread i occupied)
    let readers = [ 0..reader_count-1 ] |> List.map (fun i -> problem_4_2_read_thread i occupied lightswitch)
    writers |> List.iter _.Start()
    readers |> List.iter _.Start()
    
    writers |> List.iter _.Join()
    readers |> List.iter _.Join()