use flume::{bounded, Receiver, Sender};
use futures::StreamExt;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

enum Event {
    CtrlC,
}

fn set_up_ctrlc_handler() -> Result<Receiver<Event>, Box<dyn std::error::Error>> {
    // 创建一个有界通道，用于发送和接收 Ctrl+C 信号
    let (ctrlc_tx, ctrlc_rx) = bounded(1);

    // 标记是否已经发送过 Ctrl+C 信号
    // let ctrlc_sent = Arc::new(std::sync::atomic::AtomicBool::new(false));
    // let ctrlc_sent_clone = ctrlc_sent.clone();
    let mut ctrlc_sent = false;
    // 设置 Ctrl+C 信号的处理函数
    ctrlc::set_handler(move || {
        if ctrlc_sent {
            // 如果已经收到过 Ctrl+C 信号，则立即终止程序
            eprintln!("received second ctrlc signal -> aborting immediately");
            std::process::abort();
        } else {
            // 否则，发送 Ctrl+C 事件到通道中
            eprintln!("received ctrlc signal");
            if let Err(e) = ctrlc_tx.send(Event::CtrlC) {
                eprintln!("failed to report ctrl-c event to flume channel: {:?}", e);
            }
            ctrlc_sent = true;
        }
    })
    .map_err(|err| format!("failed to set ctrl-c handler: {}", err))?;

    // 创建一个异步流，用于监听 Ctrl+C 事件
    Ok(ctrlc_rx)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置 Ctrl+C 信号处理
    let ctrlc_rx = set_up_ctrlc_handler()?;

    // 在主线程中，使用异步流监听 Ctrl+C 事件
    let mut ctrlc_stream = ctrlc_rx.into_stream();
    while let Some(event) = ctrlc_stream.next().await {
        match event {
            Event::CtrlC => {
                println!("Ctrl+C 事件已收到，执行清理操作并退出...");
                // 在此执行需要的清理操作
                thread::sleep(Duration::from_millis(2000));
                break;
            }
        }
    }

    // 在这里可以执行其他操作

    println!("程序正常退出。");
    Ok(())
}
