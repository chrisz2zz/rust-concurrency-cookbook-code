use std::{sync::mpsc::channel, time::Duration};

fn main() {
    scheduled_thread_pool();
}

fn scheduled_thread_pool() {
    let (sender, receiver) = channel();
    let pool = scheduled_thread_pool::ScheduledThreadPool::new(4);
    let _handle = pool.execute_after(Duration::from_secs(1), move || {
        println!("Hello from a scheduled thread!");
        sender.send("done").unwrap();
    });

    pool.execute_at_dynamic_rate(Duration::from_secs(1), || {
        println!("Hello from a scheduled thread!");
        Some(Duration::from_secs(1))
    });

    receiver.recv().unwrap();

    loop {}
}
