#![feature(async_closure)]
use std::{
    sync::{atomic::AtomicI32, Arc},
    thread,
    time::Duration,
};

use chapter_02_thread_pool::*;

fn main() {
    rusty_pool_example();
    rusty_pool_wait_handle();
    rusty_pool_example2();
    rusty_pool_example3();
}

fn rusty_pool_example() {
    let pool = rusty_pool::ThreadPool::default();

    for _ in 1..10 {
        pool.execute(|| {
            println!("Hello from a rusty_pool!");
        });
    }

    pool.join();
}

fn rusty_pool_wait_handle() {
    let pool = rusty_pool::ThreadPool::default();
    let handle = pool.evaluate(|| {
        thread::sleep(Duration::from_secs(5));
        return 4;
    });

    let result = handle.await_complete();
    assert_eq!(result, 4);
}

fn rusty_pool_example2() {
    let pool = rusty_pool::ThreadPool::default();

    let closure = async move |x: i32, y: i32| -> i32 { x + y };

    let handle = pool.complete(async move {
        let a = closure(4, 6).await;
        let b = closure(a, 3).await;
        let c = closure(b, -a).await;
        closure(c, 5).await
    });

    assert_eq!(handle.await_complete(), 8);

    let count = Arc::new(AtomicI32::new(0));
    let clone = count.clone();
    pool.spawn(async move {
        let a = closure(3, 6).await;
        let b = closure(a, -4).await;
        let c = closure(b, 7).await;
        clone.fetch_add(c, std::sync::atomic::Ordering::SeqCst);
    });
    pool.join();
    assert_eq!(count.load(std::sync::atomic::Ordering::SeqCst), 12);
}

fn rusty_pool_example3() {
    let pool = rusty_pool::ThreadPool::default();
    for _ in 0..10 {
        pool.execute(|| thread::sleep(Duration::from_secs(10)));
    }

    pool.join_timeout(Duration::from_secs(5));
    let count = Arc::new(AtomicI32::new(0));
    for _ in 0..15 {
        let clone = count.clone();
        pool.execute(move || {
            thread::sleep(Duration::from_secs(5));
            clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
    }

    pool.shutdown_join();
    assert_eq!(count.load(std::sync::atomic::Ordering::SeqCst), 15);
}
