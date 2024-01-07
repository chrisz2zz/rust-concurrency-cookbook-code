use std::cell::RefCell;

use chapter_01_thread::*;

fn main() {
    start_threads_with_threadlocal();
}

pub fn start_threads_with_threadlocal() {
    // init 1
    thread_local!(static COUNTER: RefCell<u32> = RefCell::new(1));

    COUNTER.with(|c| {
        *c.borrow_mut() = 2; // change to 2
    });

    let handle1 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 3; // change to 3
        });
    });

    COUNTER.with(|c| {
        // current thread local is 2
        println!("Hello from a thread7, c={}!", *c.borrow());
    });

    let handle2 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 4; // change to 4
        });

        COUNTER.with(|c| {
            // current thread local is 4
            println!("Hello from a thread8, c={}!", *c.borrow());
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    COUNTER.with(|c| {
        // current thread local is 2
        println!("Hello from main, c={}!", *c.borrow());
    })
}
