use std::{thread, time};
use std::borrow::Borrow;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// A thread safe and easy to share queue
struct SafeQueue<T> {
    //In this way, our Queue is a Send, and Sync’s
    queue: Arc<Mutex<Vec<T>>>,
}

// use Send Clone for fix double Arc
impl<T> Clone for SafeQueue<T> {
    fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
        }
    }
}

impl<T> SafeQueue<T> {
    // Create a safe queue
    // The Vec of the queue implements send, Sync Trait
    // and wrapped by Mutex
    fn new() -> SafeQueue<T> {
        SafeQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }

    fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(item);
    }

    fn pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop()
    }
}

/// Task Structure that represents a task that can be executed
/// This task is represented by a callback function that implements the FnOnce trait
struct Task<F>
    where
        F: FnOnce() -> (),
        F: Send + 'static,
{
    callback: F,
}

impl<F:> Task<F>
    where F: FnOnce() -> (),
          F: Send + 'static,
{
    /// new Method Create a new Task, which accepts a handler function and encapsulates it in Box
    /// so that it can be stored in the structure
    /// Here is a function or closure of this function to implement the Fnonce Trait
    /// Where to limit the need is a closure similar to the closure used in spawn
    fn new(callback: F) -> Self
    {
        Task {
            callback: callback,
        }
    }

    fn run(self) {
        (self.callback)();
    }
}

/// Scheduler structure represents a coroutine scheduling, which maintains a set of work threads and task queues.
/// worker: worker thread list
/// task_queue: The task queue that can be shared safely in multiple workers threads
struct Scheduler<F>
    where
        F: FnOnce() -> (),
        F: Send + 'static,
{
    // Worker thread queue
    workers: Vec<Worker>,
    // Task queues, which are called Send and Sync, can be shared in work
    task_queue: SafeQueue<Task<F>>,
}

impl<F> Scheduler<F>
    where F: FnOnce() -> (),
          F: Send + 'static,
{
    /// According to the expected number of workers' threads, create working threads
    /// The maximum number of threads should be less than the maximum number of computer threads, because Rust cannot achieve green threads
    fn new(worker_count: usize) -> Self {
        let mut workers = Vec::new();
        // Create a new global safe task queue
        let task_queue = SafeQueue::new();

        // Cycle worker_count times, each time creates a new worker instance, and adds the safety queue created by the scheduler to
        for id in 0..worker_count {
            // Each time we create a worker, pass the task queue to worker
            workers.push(Worker::new(id, task_queue.clone()));
        }

        Scheduler {
            workers,
            task_queue,
        }
    }
    /// run method, allow the caller
    /// This task is a function or closure of Fnonce() Trait
    fn run(&mut self, task: F)
    {
        self.task_queue.push(Task::new(task));
    }
}

/// Workers
/// id: Worker ID
/// thread: Worker thread handle
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// new Function Requires a queue that can be shared
    /// and it will put the queue's send to the sub -thread
    /// Return to worker, which contains the handle of the work sub-thread and the worker ID
    fn new<F>(id: usize, task_queue: SafeQueue<Task<F>>) -> Self
        where
            F: FnOnce() -> (),
            F: Send + 'static,
    {
        let thread = thread::spawn(move || loop {
            let task = task_queue.pop();
            match task {
                Some(task) => task.run(),
                None => break,
            }
        });
        Worker { id, thread }
    }
}


mod tests {
    use super::*;

    /// test case for String
    #[test]
    fn test_string_queue() {
        // Create a shared queue to store strings and convert the shared queue to Arc smart Pointers
        let queue = Arc::new(SafeQueue::<String>::new());

        // Create a child thread. We use move here. Since our queue is Arc, the move is actually a clone
        let queue_clone = queue.clone();
        thread::spawn(move || {
            for i in 0..100 {
                queue_clone.push("Send from sender1:".to_owned() + &i.to_string());
            }
        });

        let queue_clone = queue.clone();
        thread::spawn(move || {
            for i in 0..100 {
                queue_clone.push("Send from sender2:".to_owned() + &i.to_string());
            }
        });

        let mut num = 0;

        let queue_clone = queue.clone();
        thread::spawn(move || {
            loop {
                println!("Get From Thread {:?}", queue_clone.pop());
                thread::sleep(time::Duration::from_millis(1));
            }
        });
        loop {
            thread::sleep(time::Duration::from_millis(1));
            if num < 100 {
                // Sending data to the queue in the main thread
                queue.push("Send from main:".to_string() + &num.to_string());
                num += 1;
            }
            if queue.empty() {
                break;
            }
            println!("Get From main {:?}", queue.pop());
        }
    }

    /// test case for dyn FnOnce
    #[test]
    fn test_fn_once_queue() {
        let queue = Arc::new(SafeQueue::<Box<dyn FnOnce() + Send + Sync>>::new());

        let queue_clone = queue.clone();
        thread::spawn(move || {
            for i in 0..100 {
                queue_clone.push(Box::new(move || println!("Send from sender1:{}", &i.to_string())));
            }
        });


        let queue_clone = queue.clone();
        thread::spawn(move || {
            for i in 0..100 {
                queue_clone.push(Box::new(move || println!("Send from sender2:{}", &i.to_string())));
            }
        });

        let mut num = 0;

        let queue_clone = queue.clone();
        thread::spawn(move || {
            loop {
                if !queue_clone.empty() {
                    print!("Receive from thread:{:?} , ", (queue_clone.pop().unwrap())());
                }
                thread::sleep(time::Duration::from_millis(1));
            }
        });
        loop {
            thread::sleep(time::Duration::from_millis(1));
            if num < 100 {
                queue.push(Box::new(move || println!("Send from main:{}", &num.to_string())));
                num += 1;
            }
            if queue.empty() {
                break;
            }
            if !queue.empty() {
                print!("Receive from main:{:?} , ", (queue.pop().unwrap())());
            }
        }
    }

    #[test]
    fn test_schedule() {
        // Create a thread pool and start three worker threads
        let mut scheduler = Scheduler::<Box<dyn FnOnce() + Send>>::new(3);

        // Push the task into the thread pool
        scheduler.run(Box::new(move || {
            println!("Hello from task 1!");
        }));
        scheduler.run(Box::new(move || {
            println!("Hello from task 2!");
        }));

        thread::sleep(time::Duration::from_millis(1));
    }
}