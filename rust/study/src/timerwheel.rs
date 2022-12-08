use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime};

/// 一共有两种类型的事件类型
/// Timer 是一种循环执行的事件
/// DateTime 是一种只执行一次的事件
#[derive(Eq, PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
enum EventType {
    Timer,
    DateTime,
}

/// 事件结构体
/// event_type 事件类型
/// callback 执行函数
///
#[derive(Clone)]
struct Event {
    event_type: EventType,
    callback: Rc<dyn Fn()>,
    timeout: u128,
    datetime: u128,
    delay: u128,
    id: u64,
}


impl Event {
    /// eventType: 事件类型
    /// callback: 回调函数
    /// cron: 排程
    fn new(event_type: EventType, callback: Rc<dyn Fn()>, cron: u128) -> Self {
        let id = Self::generate_id();
        match event_type {
            EventType::Timer => Event {
                event_type,
                callback,
                timeout: cron,
                datetime: 0,
                delay: cron,
                id,
            },
            EventType::DateTime => Event {
                event_type,
                callback,
                timeout: 0,
                datetime: cron,
                delay: 0,
                id,
            }
        }
    }
    /// 基于计数的方式实现id生成
    fn generate_id() -> u64 {
        // 使用一个原子计数器来生成 ID
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        id
    }
}

/// 为Event 实现 PartialEq trait
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.event_type == other.event_type
            && self.id == other.id
            && self.timeout == other.timeout
    }
}

/// 为Event 实现 PartialOrd trait
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.delay.cmp(&other.delay))
    }
}


/// 时间轮
/// events: 所有添加的事件列表
/// interval: 一轮的时间跨度
/// tickMs: 时间轮间隔
/// slots:时间槽
struct TimerWheel {
    events: Vec<Event>,
    interval: u128,
    ticks: u64,
    slots: Vec<Vec<Event>>,
}

impl TimerWheel {
    fn new(interval: u128, ticks: u64, slot_count: u32) -> Self {
        let mut wheel = TimerWheel {
            events: Vec::new(),
            interval,
            ticks,
            slots: Vec::new(),
        };

        // 初始化一定数量的时间槽
        for _ in 0..slot_count {
            wheel.slots.push(Vec::new());
        }

        wheel
    }

    /// 向时间轮中添加event
    /// event 需求一个引用
    /// 如果该event 是一个  EventType::DateTime 类型，那么会计算其 delay 值
    fn add_event(&mut self, event: Event) {
        let ms = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        let mut add_event = event.clone();
        if event.event_type == EventType::DateTime && event.datetime >= ms {
            add_event.delay = add_event.datetime - ms;
        }
        // 其实只需要调整第一个slot,所以只需要判断event的delay是否在最近的slot中
        if add_event.delay < self.interval {
            // slot中存储的clone
            self.slots[0].push(add_event.clone());
        }
        // 在push时，同时进行插入排序，调整slots中的顺序
        self.events.push(add_event);
    }

    /// 移除时间
    fn remove_event(&mut self, event: &Event) -> bool {
        let index = self
            .events
            .iter()
            .position(|e| e == event)
            .expect("event not found");
        self.events.remove(index);
        true
    }

    fn tick(&mut self) {
        // 移动时间槽,获取当前需要处理的事件列表
        let mut expired_events = self.slots.remove(0);
        self.slots.push(Vec::new());
        // 处理当前待处理事件
        for event in expired_events {
            // 执行回调函数
            (&event.callback)();
            // 移除过期的
            if event.event_type == EventType::DateTime {
                self.remove_event(&event);
            }
        }

        // 更新事件超时时间
        // 这里是 存储的过期时间，实际上可以使用 时间戳，这样就不用全部遍历
        // 并且 此处也可以使用有序列表
        for event in self.events.iter_mut() {
            // 对于所有的event，减去ticks时间
            if event.delay >= 0 {
                // 减去tricks的间隔时间
                let sub = event.delay.checked_sub(self.ticks as u128);
                match sub {
                    Some(x) if x > 0 => { event.delay = x }
                    _ => {
                        event.delay = 0;
                        // 对于 timer类型的event，如果已经过期
                        if event.event_type == EventType::Timer {
                            event.delay = event.timeout
                        }
                    }
                }
            }
        }

        // 收集所有delay在 interval 内的，添加到slot 0 中
        for event in self.events.iter().filter(|e| e.delay <= self.interval) {
            self.slots[0].push(event.clone());
        }
    }
}


#[test]
fn test() {
    println!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());

    // 每个槽时间跨度为1000ms
    // 每个槽执行间隔为1000ms
    // 一个周期为86400*7即一周
    let mut timer_wheel = TimerWheel::new(1000, 1000, 86400 * 7);
    // 创建一个 循环执行的 event，它每5000ms执行一次
    let timer_event = Event::new(EventType::Timer, Rc::new(|| println!("0 Hello, world! {:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())), 5000);
    timer_wheel.add_event(timer_event);    // 创建一个 循环执行的 event，它每5000ms执行一次
    let timer_event = Event::new(EventType::Timer, Rc::new(|| println!("1 Hello, world! {:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())), 2001);
    timer_wheel.add_event(timer_event);
    // 创建一个定时执行的 event，它将在当前时间3000ms后执行
    let datetime_event = Event::new(EventType::DateTime,
                                    Rc::new(|| println!("2 Hello, world! {:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())),
                                    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() + 3000);
    timer_wheel.add_event(datetime_event);
    // 循环执行
    loop {
        // 打印当前时间
        timer_wheel.tick();
        println!("SLEEP {:?}", timer_wheel.ticks);
        thread::sleep(Duration::from_millis(timer_wheel.ticks));
    }
}