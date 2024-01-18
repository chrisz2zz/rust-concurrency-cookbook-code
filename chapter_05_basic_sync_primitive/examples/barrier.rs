use std::{
    sync::{Arc, Barrier},
    thread,
};

use rand::Rng;

fn main() {
    example1();
    example2();
}

fn example1() {
    let barrier = Arc::new(Barrier::new(3));

    let mut handles = vec![];

    for id in 0..3 {
        let barrier = barrier.clone();
        let handle = thread::spawn(move || {
            println!("Thread {} working", id);
            thread::sleep(std::time::Duration::from_secs(id as u64));

            barrier.wait();

            println!("Thread {} resumed", id);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn example2() {
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];

    for _ in 0..10 {
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait1");
            let dur = rand::thread_rng().gen_range(1..4);
            thread::sleep(std::time::Duration::from_secs(dur));

            barrier.wait();
            println!("after wait1");
            thread::sleep(std::time::Duration::from_secs(1));

            barrier.wait();
            println!("after wait2");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
