use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let data = Arc::new(46);

    let thread1 = {
        let data = data.clone();
        thread::spawn(move || println!("Thread 1: {}", data))
    };

    let thread2 = {
        let data = data.clone();
        thread::spawn(move || println!("Thread 2: {}", data))
    };

    thread1.join().unwrap();
    thread2.join().unwrap();

    arc_with_mutex();
}

fn arc_with_mutex() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}
