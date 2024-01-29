use std::{
    sync::{
        atomic::{AtomicBool, AtomicI32},
        Arc,
    },
    thread,
};

fn main() {
    relaxed();
    acquire();
}

fn relaxed() {
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.load(std::sync::atomic::Ordering::Relaxed));
}

fn acquire() {
    let atomic_bool = Arc::new(AtomicBool::new(false));

    let atomic_bool2 = atomic_bool.clone();
    let producer =
        thread::spawn(move || atomic_bool2.store(true, std::sync::atomic::Ordering::Release));

    let consumer =
        thread::spawn(move || while !atomic_bool.load(std::sync::atomic::Ordering::Acquire) {});

    producer.join().unwrap();
    consumer.join().unwrap();
}
