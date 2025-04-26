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
    (reader_count_mutex: Semaphore)
    (active_readers: Ref<int>)
    =
    Thread(fun () ->
        while true do
            toggleSem reader_count_mutex Wait
            if active_readers.Value = 0 then toggleSem occupied Wait
            Console.WriteLine $"{label} reader writing {active_readers.Value + 1} to read_count"
            active_readers.Value <- active_readers.Value + 1
            toggleSem reader_count_mutex Release
            
            Console.WriteLine $"{label} reader reading"
            Thread.Sleep(300)
            
            toggleSem reader_count_mutex Wait
            let switching_off = active_readers.Value = 1
            active_readers.Value <- active_readers.Value - 1
            if switching_off then
                Console.WriteLine $"{label} reader last one out"
                toggleSem occupied Release
                
            toggleSem reader_count_mutex Release
    )
    
let problem_4_2 =
    let reader_count = 2
    let writer_count = 2
    let occupied = new Semaphore(1, 1)
    let reader_count_mutex = new Semaphore(1, 1)
    let active_readers = ref 0
    
    let writers = [ 0..reader_count-1 ] |> List.map (fun i -> problem_4_2_write_thread i occupied)
    let readers = [ 0..writer_count-1 ] |> List.map (fun i -> problem_4_2_read_thread i occupied reader_count_mutex active_readers)
    writers |> List.iter _.Start()
    readers |> List.iter _.Start()
    
    writers |> List.iter _.Join()
    readers |> List.iter _.Join()
    