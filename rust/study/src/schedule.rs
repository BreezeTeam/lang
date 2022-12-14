use std::{thread, time};
use std::borrow::Borrow;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// A thread safe and easy to share queue
#[derive(Clone)]
struct SafeQueue<T> {
    //In this way, our Queue is a Send, and Sync’s
    queue: Arc<Mutex<Vec<T>>>,
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

/// Task 结构体，用于表示一个可以被执行的任务
/// 该 任务由一个回调函数表示，该回调函数实现了 FnOnce trait
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
    /// new 方法创建一个 新的 Task，该方法接受一个handler函数，并将其封装在Box中
    /// 以便可以存储在结构体中
    /// 这里要求这个 函数 是实现了FnOnce trait的一个函数或者闭包
    /// where 限制需求的是一个类似与用在 spawn中的 闭包
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

/// Scheduler 结构体表示协程调度器，它维护了一组工作线程和任务队列。
/// workers 工作者线程列表
/// task_queue：可以在多个工作者线程中安全共享的任务队列
struct Scheduler<F>
    where
        F: FnOnce() -> (),
        F: Send + 'static,
{
    // Worker thread queue
    workers: Vec<Worker>,
    // Task queues, which are called Send and Sync, can be shared in work
    task_queue: Arc<SafeQueue<Task<F>>>,
}

impl<F> Scheduler<F>
    where F: FnOnce() -> (),
          F: Send + 'static,
{
    /// 根据 预计工作者线程熟练数量，创建工作线程
    /// 线程的最大数量应该小于计算机最大线程数，因为rust 还无法实现绿色线程
    fn new(worker_count: usize) -> Self {
        let mut workers = Vec::new();
        // 创建一个新的任务队列
        let task_queue = Arc::new(SafeQueue::new());

        // 循环 worker_count次，每次创建一个新的Worker实例，并且将调度器创建的安全队列其添加到 workers中
        for id in 0..worker_count {
            // 每次创建Worker，将该任务队列传递给worker
            workers.push(Worker::new(id, task_queue.clone()));
        }

        // 返回scheduler
        Scheduler {
            workers,
            task_queue,
        }
    }
    /// run 方法，允许调用者传递一个任务，并且将其包装为Task后添加到队列中
    /// 该任务是一个实现了 FnOnce() trait 的函数或者闭包
    fn run(&mut self, task: F)
    {
        self.task_queue.push(Task::new(task));
    }
}


/// 工作者
/// id: 工作者id
/// thread: 工作者线程句柄
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// new 函数 需求一个 可以共享的 队列
    /// 并且它会将该队列的Send到子线程中
    /// 返回 Worker，里面包含了 工作子线程的句柄 以及工作者id
    fn new<F>(id: usize, task_queue: Arc<SafeQueue<Task<F>>>) -> Self
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