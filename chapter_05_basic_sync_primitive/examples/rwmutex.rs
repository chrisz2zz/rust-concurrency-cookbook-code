use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    example1();
    example2();
}

fn example1() {
    let counter = Arc::new(RwLock::new(0));

    let mut read_handles = vec![];
    for _ in 0..3 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let num = counter.read().unwrap();
            println!("Reader: {:?}: {}", thread::current().id(), *num);
        });

        read_handles.push(handle);
    }

    let write_handle = thread::spawn(move || {
        let mut num = counter.write().unwrap();
        *num += 1;
        println!(
            "Writer: {:?}: Incremented counter to {}",
            thread::current().id(),
            *num
        );
    });

    for handle in read_handles {
        handle.join().unwrap();
    }

    write_handle.join().unwrap()
}

fn example2() {
    let counter = Arc::new(RwLock::new(0));

    let read_handle = {
        let counter = counter.clone();
        thread::spawn(move || {
            let num = counter.read().unwrap();

            println!("Reader: {:?}: {}", thread::current().id(), *num);

            thread::sleep(std::time::Duration::from_secs(10));
        })
    };

    let write_handle = {
        let counter = counter.clone();
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(1));

            let mut num = counter.write().unwrap();
            *num += 1;
            println!(
                "Writer: {:?}: Incremented counter to {}",
                thread::current().id(),
                *num
            );
        })
    };

    read_handle.join().unwrap();
    write_handle.join().unwrap();
}
