open System.Threading
open System.Collections.Generic
open System.Threading
open System

let problem_3_8_thread
    (internal_sem: Semaphore)
    (external_sem: Semaphore)
    (dancer_list: Queue<String>)
    (label: String)
    =
    Thread(fun () ->
        Console.WriteLine("here")

        while true do
            if dancer_list.Count <> 0 then
                Console.WriteLine $"{label} thread waiting"
                internal_sem.Release() |> ignore
                external_sem.WaitOne() |> ignore
                Console.WriteLine $"Dancer: {dancer_list.Dequeue()}")


let problem_3_8 () =
    let leader_sem = new Semaphore(0, 1)
    let follower_sem = new Semaphore(0, 1)
    let follow_list = new Queue<String>()
    let leader_list = new Queue<String>()

    leader_list.Enqueue("leader1")
    leader_list.Enqueue("leader2")
    leader_list.Enqueue("leader3")
    leader_list.Enqueue("leader4")

    let leaders = problem_3_8_thread leader_sem follower_sem leader_list "leader"

    let follwers = problem_3_8_thread follower_sem leader_sem follow_list "follower"

    follow_list.Enqueue("follower1")
    follow_list.Enqueue("follower2")
    follow_list.Enqueue("follower3")

    leaders.Start()
    follwers.Start()
    leaders.Join()
    follwers.Join()
    0

problem_3_8 ()
