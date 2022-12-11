use std::{thread, time};
use std::sync::{Arc, Mutex};

/// 一个线程安全易于共享的队列
#[derive(Clone)]
struct SafeQueue<T>
    where T: Send, {
    // 这样我们的queue就是一个Send,并且Sync的
    queue: Arc<Mutex<Vec<T>>>,
}

impl<T: Send> SafeQueue<T> {
    // 创建一个安全的队列
    // 该队列的 Vec 实现了 Send，Sync trait
    // 并且 被 Mutex 包裹
    fn new() -> SafeQueue<T> {
        SafeQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }
    // 判断是否为空队列
    fn empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }
    // 向队列安全的推一个item
    fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(item);
    }
    // 安全的弹出一个对象
    fn pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop()
    }
}


macro_rules! go {
    ($($body:tt)*) => {{
        thread::spawn(move || {
            $($body)*
        });
    }}
}

#[test]
fn test() {
    // 创建一个存储字符串的共享队列,将共享队列转换成 Arc 智能指针
    let queue = Arc::new(SafeQueue::<String>::new());

    // 创建一个子线程
    // 这里使用了 move ,由于我们的queue 是 Arc的，所以move 的实际上是一个clone
    let queue_clone = queue.clone();
    go! {
        for i in 0..100{
            queue_clone.push("Send from sender1:".to_owned()+&i.to_string());
        }
    }

    let queue_clone = queue.clone();
    go! {
        for i in 0..100{
            queue_clone.push("Send from sender2:".to_owned()+&i.to_string());
        }
    }

    let mut num = 0;

    let queue_clone = queue.clone();
    go! {
        loop{
            println!("Get From Thread {:?}", queue_clone.pop());
            thread::sleep(time::Duration::from_millis(1));
        }
    }
    loop {
        thread::sleep(time::Duration::from_millis(1));
        if num < 100 {
            // 在主线程中向队列中发送数据
            queue.push("Send from main:".to_string() + &num.to_string());
            num += 1;
        }
        if queue.empty() {
            break;
        }
        println!("Get From main {:?}", queue.pop());
    }
}