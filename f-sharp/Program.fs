open System.Threading
open System.Collections.Generic
open System.Threading.Tasks
open System

type DancerType = Leader | Follower
type ConcurrencyType = ThreadModel | TaskModel

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

// `54d86e2`: held at first task, why?
let problem_3_8_task (internal_sem: Semaphore) (external_sem: Semaphore) (dancer_list: Queue<String>) (label: String) =
    task {
        while true do
            if dancer_list.Count <> 0 then
                Console.WriteLine $"{label} thread waiting"
                internal_sem.Release() |> ignore
                external_sem.WaitOne() |> ignore
                Console.WriteLine $"Dancer: {dancer_list.Dequeue()}"
    }


let problem_3_8 concurrency_model =
    let leader_sem = new Semaphore(0, 1)
    let follower_sem = new Semaphore(0, 1)
    let follow_list = new Queue<String>()
    let leader_list = new Queue<String>()

    leader_list.Enqueue("leader1")
    leader_list.Enqueue("leader2")
    leader_list.Enqueue("leader3")
    leader_list.Enqueue("leader4")

    match concurrency_model with
    | ThreadModel ->
        let leaders = problem_3_8_thread leader_sem follower_sem leader_list "leader"
        let follwers = problem_3_8_thread follower_sem leader_sem follow_list "follower"
        leaders.Start()
        follwers.Start()

        follow_list.Enqueue("follower1")
        follow_list.Enqueue("follower2")
        follow_list.Enqueue("follower3")

        follwers.Join()
    | TaskModel ->
        let leaders = problem_3_8_task leader_sem follower_sem leader_list "leader"
        let follwers = problem_3_8_task follower_sem leader_sem follow_list "follower"
        leaders.Start()
        follwers.Start()


//problem_3_8 "tasks"

let problem_3_8_thread_alt  (leaderQueue: Semaphore) (followerQueue: Semaphore) (dancerType: string) (id: int) =
    Thread(fun () ->
        Console.WriteLine($"{dancerType} {id} has arrived")

        match dancerType with
        | "leader" ->
            leaderQueue.Release() |> ignore
            Console.WriteLine($"{dancerType} {id} is waiting")
            followerQueue.WaitOne() |> ignore
            Console.WriteLine($"{dancerType} {id} paired with a follower")
        | _ ->
            followerQueue.Release() |> ignore
            Console.WriteLine($"{dancerType} {id} is waiting")
            leaderQueue.WaitOne() |> ignore
            Console.WriteLine($"{dancerType} {id} paired with a leader")
            Thread.Sleep(0))
let problem_3_8_task_alt (leaderQueue: Semaphore) (followerQueue: Semaphore) (dancerType: string) (id: int) =
    task {
        Console.WriteLine($"{dancerType} {id} has arrived")

        match dancerType with
        | "leader" ->
            leaderQueue.Release() |> ignore
            Console.WriteLine($"{dancerType} {id} is waiting")
            followerQueue.WaitOne() |> ignore
            Console.WriteLine($"{dancerType} {id} paired with a follower")
        | _ ->
            followerQueue.Release() |> ignore
            Console.WriteLine($"{dancerType} {id} is waiting")
            leaderQueue.WaitOne() |> ignore
            Console.WriteLine($"{dancerType} {id} paired with a leader")
            do! Task.Delay(0)
    }
let problem_3_8_alt concurrency_model = 
    // Claude initially halucinated new Semaphore(0,100)
    let leaderQueue = new Semaphore(0, 3)
    let followerQueue = new Semaphore(0, 3)

    let dancers = [ ("leader", 1);
          ( "follower", 1);
          ( "leader", 2);
          ( "leader", 3);
          ( "follower", 2);
          ( "follower", 3) ]
         
         
    match concurrency_model with
    | TaskModel ->
        let threads = dancers |> List.map (fun d -> d ||> problem_3_8_thread_alt leaderQueue followerQueue)
        threads |> List.iter _.Start()
        threads |> List.iter _.Join()
    | ThreadModel ->
        let tasks = dancers |> List.map (fun d -> d ||> problem_3_8_task_alt leaderQueue followerQueue)
        Task.WhenAll tasks |> ignore

problem_3_8_alt TaskModel