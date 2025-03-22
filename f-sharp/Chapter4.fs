module Chapter4
open Common
open System
open System.Threading

type IoState =
    | Reading
    | Writing

let problem_4_2_reader
    (mutexSem: Semaphore)
    (ioState: IoState)
    (readerCount: int)
    (writerCount: int)
    (label: String)
    =
    
    toggleSem mutexSem Wait
    
    match (ioState, readerCount, writerCount) with
        | (Reading, _, 0)  -> ignore
        | (Writing, 0, 0) -> ignore
        | (Writing, 0, _) -> ignore
        | (Reading, _, writers) when writers > 0 -> raise (Exception("Illegal"))
        | (Writing, readers, _) when readers > 0 -> raise (Exception("Illegal"))
        
    
    