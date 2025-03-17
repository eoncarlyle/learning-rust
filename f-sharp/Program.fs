open System.Threading
open System.Collections.Generic
open System.Threading.Tasks
open System

type DancerType =
    | Leader
    | Follower

type ConcurrencyType =
    | ThreadModel
    | TaskModel

type SemaphoreOperation =
    | Release
    | Wait

let toggleSem (sem: Semaphore) (semaphoreOperation: SemaphoreOperation) =
    match semaphoreOperation with
    | Release -> sem.Release() |> ignore
    | Wait -> sem.WaitOne() |> ignore

let problem_3_8_thread
    (internal_sem: Semaphore)
    (external_sem: Semaphore)
    (dancer_list: Queue<String>)
    (label: String)
    =
    Thread(fun () ->
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

    | TaskModel ->
        let leaders = problem_3_8_task leader_sem follower_sem leader_list "leader"
        let follwers = problem_3_8_task follower_sem leader_sem follow_list "follower"
        leaders.Start()
        follwers.Start()


//problem_3_8 "tasks"

let problem_3_8_thread_alt (leaderQueue: Semaphore) (followerQueue: Semaphore) (dancerType: string) (id: int) =
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

    let dancers =
        [ ("leader", 1)
          ("follower", 1)
          ("leader", 2)
          ("leader", 3)
          ("follower", 2)
          ("follower", 3) ]


    match concurrency_model with
    | TaskModel ->
        let threads =
            dancers
            |> List.map (fun d -> d ||> problem_3_8_thread_alt leaderQueue followerQueue)

        threads |> List.iter _.Start()
        threads |> List.iter _.Join()
    | ThreadModel ->
        let tasks =
            dancers
            |> List.map (fun d -> d ||> problem_3_8_task_alt leaderQueue followerQueue)

        Task.WhenAll tasks |> ignore


let problem_3_8_provided_thread_leaders
    (leaders: int)
    (followers: int)
    (mutexSem: Semaphore)
    (leaderQueue: Semaphore)
    (followerQueue: Semaphore)
    (rendezvous: Semaphore)
    =
    Thread(fun () ->
        mutexSem.WaitOne() |> ignore

        if followers > 0 then
            let followRef = ref followers
            followRef.Value <- followers - 1
            followerQueue.Release() |> ignore
        else
            let leadersRef = ref leaders
            leadersRef.Value <- leaders + 1
            mutexSem.Release() |> ignore
            leaderQueue.WaitOne() |> ignore

        Console.WriteLine $"Dancing Leader"
        rendezvous.WaitOne() |> ignore
        mutexSem.Release() |> ignore)

let problem_3_8_provided_thread_followers
    (leaders: int)
    (followers: int)
    (mutexSem: Semaphore)
    (leaderQueue: Semaphore)
    (followerQueue: Semaphore)
    (rendezvous: Semaphore)
    =
    Thread(fun () ->
        mutexSem.WaitOne() |> ignore

        if followers > 0 then
            let followRef = ref followers
            followRef.Value <- followers - 1
            followerQueue.Release() |> ignore
        else
            let leadersRef = ref leaders
            leadersRef.Value <- leaders + 1
            mutexSem.Release() |> ignore
            leaderQueue.WaitOne() |> ignore

        Console.WriteLine $"Dancing Follower"
        rendezvous.Release() |> ignore)


let problem_3_8_provided () =
    let followCount = 3
    let leaderCount = followCount
    let mutexSem = new Semaphore(0, followCount)
    let leaderQueue = new Semaphore(0, followCount)
    let followerQueue = new Semaphore(0, followCount)
    let rendezvous = new Semaphore(0, followCount)

    let threads =
        seq { 0..3 }
        |> Seq.map (fun index ->
            if index % 2 = 0 then
                problem_3_8_provided_thread_followers
            else
                problem_3_8_provided_thread_followers)
        |> Seq.map (fun a -> a leaderCount followCount mutexSem leaderQueue followerQueue rendezvous)

    threads |> Seq.iter _.Start()
    Thread.Sleep(3)
    threads |> Seq.iter _.Join()

//let problem_3_8_provided_thread_dancer (leaders: int) (followers: int) (leaderQueue: Semaphore) (followerQueue: Semaphore) (rendezvous: Semaphore) =

//problem_3_8_alt TaskModel
//problem_3_8_provided ()
problem_3_8 ConcurrencyType.ThreadModel
