enum EventType {
    Timer,
    Signal,
    IO,
}

struct Event {
    event_type: EventType,
    callback: Box<dyn Fn()>,
    timeout: u64,
}

struct TimerWheel {
    events: Vec<Event>,
    interval: u64,
    slots: Vec<Vec<Event>>,
}

///这是一个简单的时间轮实现，它可以接收不同类型的事件并在超时时触发事件的回调函数。请注意，这仅仅是一个示例代码，它并不能直接用于生产环境。如果您想要在生产环境中使用时间轮，建议您使用现成的时间轮库，例如 tokio-timer 和 wheel-timer 等。
/// 
/// 
impl TimerWheel {
    fn new(interval: u64) -> Self {
        let mut wheel = TimerWheel {
            events: Vec::new(),
            interval,
            slots: Vec::new(),
        };

        // 初始化时间槽
        for _ in 0..wheel.interval {
            wheel.slots.push(Vec::new());
        }

        wheel
    }

    fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

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
        // 移动时间槽
        let mut expired_events = self.slots.remove(0);
        self.slots.push(Vec::new());

        // 处理过期事件
        for event in expired_events {
            (event.callback)();
        }

        // 更新事件超时时间
        for event in self.events.iter_mut() {
            if event.timeout > 0 {
                event.timeout -= 1;
            }
        }

        // 添加新事件
        for event in self.events.iter().filter(|e| e.timeout == 0) {
            self.slots[0].push(event.clone());
        }
    }
}

#[test]
fn test() {
    let mut timer_wheel = TimerWheel::new(100);
    let event = Event {
        event_type: EventType::Timer,
        callback: Box::new(|| {
            println!("Hello, world!");
        }),
        timeout: 1000,
    };
    timer_wheel.add_event(event);
    loop {
        timer_wheel.tick();
        // 其他代码
        // ...
    }
}