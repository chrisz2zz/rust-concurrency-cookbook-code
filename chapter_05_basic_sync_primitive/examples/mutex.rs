use std::{
    panic::catch_unwind,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    lock();
    try_lock();
    poisoning();
    unlock_faster1();
    unlock_faster2();
}

fn lock() {
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

fn try_lock() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5000 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            if let Ok(mut num) = counter.try_lock() {
                *num += 1;
            } else {
                println!("Thread failed to acquire lock");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}

fn poisoning() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = counter.clone();
        let handle = thread::spawn(move || match counter.lock() {
            Ok(mut num) => match catch_unwind(move || {
                *num += 1;
                if *num == 3 {
                    panic!("Simulated panic!");
                }
            }) {
                Ok(_) => (),
                Err(e) => println!("occur a panic, {:?}", e),
            },
            Err(poisoned) => {
                println!("Thread encountered a poisoned lock: {:?}", poisoned);

                let mut num = poisoned.into_inner();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    match counter.lock() {
        Ok(num) => println!("Final count: {}", *num),
        Err(e) => println!("Final count: {}", *e.into_inner()),
    };
}

fn unlock_faster1() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            {
                let mut num = counter.lock().unwrap();
                *num += 1;
            } // drop MutexGuard here

            // do something with mutex
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}

fn unlock_faster2() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;

            drop(num); // drop MutexGuard
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}
