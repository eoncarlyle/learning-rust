open System.Threading
open System.Collections.Generic
open System.Threading
open System

let problem_3_8_thread
    (internal_sem: SemaphoreSlim)
    (external_sem: SemaphoreSlim)
    (dancer_list: Queue<String>)
    (label: String)
    =
    Thread(fun () ->
        Console.WriteLine("here")

        while true do
            if dancer_list.Count <> 0 then
                Console.WriteLine $"{label} thread waiting"
                internal_sem.Release() |> ignore
                external_sem.Wait() |> ignore
                Console.WriteLine $"Dancer: {dancer_list.Dequeue()}")

// `54d86e2`: held at first task, why?
let problem_3_8_task
    (internal_sem: SemaphoreSlim)
    (external_sem: SemaphoreSlim)
    (dancer_list: Queue<String>)
    (label: String)
    =
    task {
        while true do
            if dancer_list.Count <> 0 then
                Console.WriteLine $"{label} thread waiting"
                internal_sem.Release() |> ignore
                external_sem.Wait() |> ignore
                Console.WriteLine $"Dancer: {dancer_list.Dequeue()}"
    }


let problem_3_8 (concurrency_model) =
    let leader_sem = new SemaphoreSlim(0, 1)
    let follower_sem = new SemaphoreSlim(0, 1)
    let follow_list = new Queue<String>()
    let leader_list = new Queue<String>()

    leader_list.Enqueue("leader1")
    leader_list.Enqueue("leader2")
    leader_list.Enqueue("leader3")
    leader_list.Enqueue("leader4")

    match concurrency_model with
    | "threads" ->
        let leaders = problem_3_8_thread leader_sem follower_sem leader_list "leader"
        let follwers = problem_3_8_thread follower_sem leader_sem follow_list "follower"
        leaders.Start()
        follwers.Start()

        follow_list.Enqueue("follower1")
        follow_list.Enqueue("follower2")
        follow_list.Enqueue("follower3")

        follwers.Join()
    | "tasks" ->
        let leaders = problem_3_8_task leader_sem follower_sem leader_list "leader"
        let follwers = problem_3_8_task follower_sem leader_sem follow_list "follower"
        leaders.Start()
        follwers.Start()


    | _ -> Console.WriteLine("Unsupported model")



problem_3_8 ("threads")

let problem_3_8_thread_alt (leaderQueue: Semaphore, followerQueue: Semaphore, dancerType: string, id: int) =
    Thread(fun () ->
        Console.WriteLine($"{dancerType} {id} has arrived")

        match dancerType with
        | "leader" ->
            if dancerType = "leader" then
                if followerQueue.WaitOne(0) then
                    Console.WriteLine($"{dancerType} {id} paired with a follower")
                else
                    Console.WriteLine($"{dancerType} {id} is waiter")
                    leaderQueue.Release() |> ignore
        | _ ->
            if leaderQueue.WaitOne(0) then
                Console.WriteLine($"{dancerType} {id} paired with a leader")
            else
                Console.WriteLine($"{dancerType} {id} is waiting")
                followerQueue.Release() |> ignore
                Thread.Sleep(0))

let problem_3_8_alt () =
    // Claude initially halucinated new Semaphore(0,100)
    let leaderQueue = new Semaphore(0, 1)
    let followerQueue = new Semaphore(0, 1)

    let dancers =
        [ problem_3_8_thread_alt (leaderQueue, followerQueue, "leader", 1)
          problem_3_8_thread_alt (leaderQueue, followerQueue, "follower", 1)
          problem_3_8_thread_alt (leaderQueue, followerQueue, "leader", 2)
          problem_3_8_thread_alt (leaderQueue, followerQueue, "leader", 3)
          problem_3_8_thread_alt (leaderQueue, followerQueue, "follower", 2)
          problem_3_8_thread_alt (leaderQueue, followerQueue, "follower", 3) ]

    dancers |> List.iter (_.Start())
    dancers |> List.iter (_.Join())

    0
