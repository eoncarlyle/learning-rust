module Chapter4
open System.ComponentModel
open Common
open System
open System.Threading

type IoState =
    | Reading
    | Writing

let problem_4_2_writer
    (mutexSem: Semaphore)
    (focusedValue: byref<int>)
    (ioState: byref<IoState>)
    (readerCount: int)
    (writerCount: byref<int>)
    (label: String)
    =
    Thread( fun () -> 
        let rand = Random()
        let mutable threadIoState = ioState
        let mutable threadWriterCount = writerCount
        let mutable threadFocusedValue = focusedValue

        let write () =
            threadWriterCount <- threadWriterCount + 1
            toggleSem mutexSem Release
            Console.WriteLine($"W{label}: Writing...")
            let writtenValue = rand.Next(0, 10)
            threadFocusedValue <- writtenValue
            Console.WriteLine($"W{label}: Wrote {writtenValue}")
            toggleSem mutexSem Wait
            threadWriterCount <- threadWriterCount - 1
            toggleSem mutexSem Release

        let steal () =
            threadIoState <- Writing
            write ()

        let wait () =
            Console.WriteLine($"W:{label} reading")
            toggleSem mutexSem Release
            Thread.Sleep(300)

        while true do
            toggleSem mutexSem Wait
            match (threadIoState, readerCount, writerCount) with
            | (Reading, 0, 0) -> steal ()
            | (Writing, 0, 0) -> write ()
            | (Reading, readers, 0) when readers > 0 -> wait ()
            | _ -> raise (Exception($"Illegal state: {ioState}, {readerCount}, {writerCount}")))

let problem_4_2_reader
    (mutexSem: Semaphore)
    (focusedValue: byref<int>)
    (ioState: byref<IoState>)
    (readerCount: byref<int>)
    (writerCount: int)
    (label: String)
    =
    
    let mutable threadIoState = ioState
    let mutable threadReaderCount = readerCount
    let mutable threadFocusedValue = focusedValue
    
    let read () =
        threadReaderCount <- threadReaderCount + 1
        toggleSem mutexSem Release
        Console.WriteLine($"R{label}: Reading...")
        Console.WriteLine($"R{label}: Read {threadFocusedValue}")
        Thread.Sleep(300)
        toggleSem mutexSem Wait
        threadReaderCount <- threadReaderCount - 1
        toggleSem mutexSem Release

    let steal () =
        threadIoState <- Reading
        read ()

    let wait () =
        Console.WriteLine($"R:{label} reading")
        toggleSem mutexSem Release
        Thread.Sleep(300)

    while true do
        toggleSem mutexSem Wait
        match (threadIoState, threadReaderCount, writerCount) with
            | (Reading, readers, 0) when readers >= 0  -> read ()
            | (Writing, 0, 0) -> steal ()
            | (Writing, 0, writers) when writers > 0 -> wait ()
            | _ -> raise (Exception($"Illegal state: {ioState}, {readerCount}, {writerCount}"))
