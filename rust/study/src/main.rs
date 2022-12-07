use std::sync::{Arc, Mutex};
use std::thread;
use std::vec::Vec;

/// 一个线程安全易于共享的队列
struct SafeQueue<T> {
    queue: Arc<Mutex<Vec<T>>>,
}

impl<T> SafeQueue<T> {
    fn new() -> SafeQueue<T> {
        SafeQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(item);
    }

    fn pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop()
    }

    fn clone(&self) -> SafeQueue<T> {
        SafeQueue {
            queue: Arc::clone(&self.queue),
        }
    }
}

///Scheduler 结构体表示协程调度器，它维护了一组工作线程和任务队列。
struct Scheduler {
    // 工作线程
    workers: Vec<Worker>,
    // 任务队列
    task_queue: SafeQueue<Task>,
}

impl Scheduler {
    fn new(worker_count: usize) -> Self {
        let mut workers = Vec::new();
        // 创建一个新的任务队列
        let task_queue = SafeQueue::new();

        // 循环 worker_count次，每次创建一个新的Worker实例，并且将其添加到
        // workers中
        for id in 0..worker_count {
            // 每次创建Worker，将该任务队列传递给worker
            workers.push(Worker::new(id, task_queue));
        }

        // 返回scheduler
        Scheduler {
            workers,
            task_queue,
        }
    }

    // run 方法，允许调用者传递一个任务，并且将其添加到队列中
    // 即 实现了 FnOnce() trait 的函数或者闭包
    fn run<F>(&mut self, task: F)
        where
            F: FnOnce() + Send + 'static,
    {
        self.task_queue.push(Task::new(task));
    }
}

/// Task 结构体，用于表示一个可以被执行的任务
/// 该 任务由一个回调函数表示，该回调函数实现了 FnOnce trait
struct Task {
    callback: Box<dyn FnOnce()>,
}

impl Task {
    // new 方法创建一个 新的 Task，该方法接受一个handler函数，并将其封装在Box中
    // 以便可以存储在结构体中
    // 这里要求这个 函数 是实现了FnOnce trait的一个函数或者闭包
    fn new<F>(callback: F) -> Self
        where
            F: FnOnce() + Send + 'static,
    {
        Task {
            callback: Box::new(callback),
        }
    }

    fn run(self) {
        (self.callback)();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, task_queue: SafeQueue<Task>) -> Self {
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

fn main() {
    // 创建线程池，并启动 3 个工作线程
    let scheduler = Scheduler::new(3);

    // 将任务推入线程池中
    scheduler.run(|| {
        println!("Hello from task 1!");
    });

    scheduler.run(|| {
        println!("Hello from task 2!");
    });
}