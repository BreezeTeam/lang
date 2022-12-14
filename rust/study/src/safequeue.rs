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
    // The VEC of the queue implements send, Sync Trait
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

/// test case for String
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

fn main() {
    test_string_queue();
    test_fn_once_queue();
}