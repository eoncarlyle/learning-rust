module Chapter4

open System.Runtime
open Common
open System.Threading
open System
open Microsoft.FSharp.Core

// Writers: if the room isn't occupied, toggle the occupancy sempahore
// Readers: keep track of the number readers, if zero then toggle off occupancy

let problem_4_2_2_write_thread (label: int) (occupied: Semaphore) =
    Thread(fun () ->
        while true do
            Console.WriteLine $"{label} writer waiting"
            toggleSem occupied Wait
            Console.WriteLine $"{label} writer writing"
            Thread.Sleep(300)
            toggleSem occupied Release)

let problem_4_2_2_read_thread (label: int) (occupied: Semaphore) (lightswitch: Lightswitch) =
    Thread(fun () ->
        while true do
            lightswitch.Lock occupied
            Console.WriteLine $"{label} reader reading"
            Thread.Sleep(300)
            //Dropping the parenthesis off of the statement below
            //F# parser limitations
            lightswitch.Unlock occupied)

let problem_4_2_2 () =
    let reader_count = 2
    let writer_count = 2
    let occupied = new Semaphore(1, 1)
    let lightswitch = Lightswitch()

    let writers =
        [ 0 .. writer_count - 1 ]
        |> List.map (fun i -> problem_4_2_2_write_thread i occupied)

    let readers =
        [ 0 .. reader_count - 1 ]
        |> List.map (fun i -> problem_4_2_2_read_thread i occupied lightswitch)

    writers |> List.iter _.Start()
    readers |> List.iter _.Start()

    writers |> List.iter _.Join()
    readers |> List.iter _.Join()

let problem_4_2_3_write_thread (label: int) (occupied: Semaphore) (nextup: Semaphore) =
    Thread(fun () ->
        for i in 1..4 do
            Console.WriteLine $"{label} writer waiting nextup"
            toggleSem nextup Wait
            Console.WriteLine $"{label} writer passed nextup"
            toggleSem occupied Wait
            toggleSem nextup Release
            Console.WriteLine $"{label} writer writing"
            Thread.Sleep(1000)
            toggleSem occupied Release)

let problem_4_2_3_read_thread (label: int) (occupied: Semaphore) (lightswitch: Lightswitch) (nextup: Semaphore) =
    Thread(fun () ->
        for i in 1..4 do
            Console.WriteLine $"\t {label} reader waiting nextup"
            toggleSem nextup Wait
            Console.WriteLine $"\t{label} reader passed nextup"
            lightswitch.Lock occupied
            toggleSem nextup Release
            Console.WriteLine $"\t{label} reader reading"
            Thread.Sleep(1000)
            lightswitch.Unlock occupied)

let problem_4_2_3 () =
    let reader_count = 2
    let writer_count = 2
    let occupied = new Semaphore(1, 1)
    let lightswitch = Lightswitch()
    let nextup = new Semaphore(1, 1)

    let writers =
        [ 0 .. writer_count - 1 ]
        |> List.map (fun i -> problem_4_2_3_write_thread i occupied nextup)

    let readers =
        [ 0 .. reader_count - 1 ]
        |> List.map (fun i -> problem_4_2_3_read_thread i occupied lightswitch nextup)

    writers |> List.iter _.Start()
    readers |> List.iter _.Start()

    writers |> List.iter _.Join()
    readers |> List.iter _.Join()



let problem_4_2_6_write_thread (label: int) (occupied: Semaphore) (turnstile: Semaphore) (writeSwitch: Lightswitch) =
    Thread(fun () ->
        for i in 1..4 do
            Console.WriteLine $"{label} writer waiting nextup"
            writeSwitch.Lock turnstile
            Console.WriteLine $"{label} writer passed nextup"
            toggleSem occupied Wait

            Console.WriteLine $"{label} writer writing"
            Thread.Sleep(1000)

            writeSwitch.Unlock turnstile
            toggleSem occupied Release)

let problem_4_2_6_read_thread
    (label: int)
    (occupied: Semaphore)
    (readSwitch: Lightswitch)
    (readerTurnstile: Semaphore)
    =
    Thread(fun () ->
        for i in 1..4 do
            Console.WriteLine $"\t{label} reader waiting nextup"

            toggleSem readerTurnstile Wait
            Console.WriteLine $"\t{label} reader passed nextup"
            readSwitch.Lock occupied
            toggleSem readerTurnstile Release

            Console.WriteLine $"\t{label} reader reading"
            Thread.Sleep(750)

            readSwitch.Unlock occupied)

let problem_4_2_6 () =
    let reader_count = 5
    let writer_count = 3
    let occupied = new Semaphore(1, 1)
    let readSwitch = Lightswitch()
    let writeSwitch = Lightswitch()
    let nextup = new Semaphore(1, 1)

    let writers =
        [ 0 .. writer_count - 1 ]
        |> List.map (fun i -> problem_4_2_6_write_thread i occupied nextup writeSwitch)

    let readers =
        [ 0 .. reader_count - 1 ]
        |> List.map (fun i -> problem_4_2_6_read_thread i occupied readSwitch nextup)

    writers |> List.iter _.Start()
    readers |> List.iter _.Start()

    writers |> List.iter _.Join()
    readers |> List.iter _.Join()
