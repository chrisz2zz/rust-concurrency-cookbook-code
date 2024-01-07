use chapter_01_thread::*;

fn main() {
    start_thread_with_sleep();
    start_thread_with_yield_now();
    thread_park_unpark();
    thread_unpark_before_park();
}

pub fn start_thread_with_sleep() {
    let handle1 = thread::spawn(|| {
        thread::sleep(std::time::Duration::from_secs(2));
        println!("Hello from a thread3!");
    });

    let handle2 = thread::spawn(|| {
        thread::sleep(std::time::Duration::from_secs(1));
        println!("Hello from a thread4!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn start_thread_with_yield_now() {
    let handle1 = thread::spawn(|| {
        println!("1");
        thread::yield_now();
        println!("yield_now!");
    });

    let handle2 = thread::spawn(|| {
        println!("2");
        thread::yield_now();
        println!("yield_now in another thread!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn thread_park_unpark() {
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();

    thread::sleep(std::time::Duration::from_millis(10));

    println!("Unpark the thread");
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
}

pub fn thread_unpark_before_park() {
    let handle = thread::spawn(|| {
        thread::sleep(std::time::Duration::from_secs(1));
        thread::park();
        println!("resume");
        thread::park();
        println!("resume");
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });

    // the parked thread will be awaken at once
    handle.thread().unpark();
    // the following unpark may not be effect, there is no guarantee that park-unpark is a one-to-one correspondence
    // the source code can show it
    handle.thread().unpark();
    handle.thread().unpark();
    handle.join().unwrap();
}
