#![feature(thread_spawn_unchecked)]
use std::thread;

fn main() {
    start_one_thread();
    start_one_thread_result();
    start_two_threads();
    start_n_threads();
    start_one_thread_by_builder();
    current_thread();
    thread_park_unpark();
    get_available_parallelism();
    get_cpu_cores();
    get_cpu_nums();
    get_current_process_thread_nums();
    start_thread_with_sleep();
    start_thread_with_yield_now();


    // must be last
    thread_unpark_before_park();
}

pub fn start_one_thread() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}

pub fn start_one_thread_result() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
        200
    });

    match handle.join() {
        Ok(v) => println!("thread result: {}", v),
        Err(e) => println!("error: {:?}", e),
    }
}

pub fn start_two_threads() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1!");
    });

    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn start_n_threads() {
    const N: isize = 10;

    let handles: Vec<_> = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread{}!", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn start_one_thread_by_builder() {
    let builder = thread::Builder::new()
        .name("foo".into()) // set thread name
        .stack_size(32 * 1024); // set thread stack size

    let handler = builder
        .spawn(|| {
            println!("Hello from a thread!");
        })
        .unwrap();
    let builder = thread::Builder::new()
        .name("foo".into()) // set thread name
        .stack_size(32 * 1024);

    let x = 1;
    let thread_x = &x;

    // `x` does not live long enough borrowed value does not live long
    // let handler2 = builder.spawn(move || {
    //     println!("Hello from inside thread! x = {}", *thread_x);
    // });

    // but this can`t check, user must ensure join() is called before current thread finish
    let handler2 = unsafe {
        builder
            .spawn_unchecked(move || {
                println!("Hello from inside thread! x = {}", *thread_x);
            })
            .unwrap()
    };

    handler2.join().unwrap();

    handler.join().unwrap();
}

pub fn current_thread() {
    let current_thread = thread::current();
    println!(
        "current thread: {:?}, {:?}",
        current_thread.id(),
        current_thread.name()
    );

    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

    let handler = builder
        .spawn(|| {
            let current_thread = thread::current();
            println!(
                "child thread: {:?}, {:?}",
                current_thread.id(),
                current_thread.name()
            );
        })
        .unwrap();

    handler.join().unwrap();
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

pub fn get_available_parallelism() {
    let count = thread::available_parallelism().unwrap().get();
    println!("available parallelism count: {}", count);
    assert!(count >= 1_usize);
}

pub fn get_cpu_cores() {
    println!(
        "cores: {:?}",
        (0..affinity::get_core_num()).collect::<Vec<usize>>()
    )
}

pub fn get_cpu_nums() {
    println!("cpu logic core nums: {}", num_cpus::get())
}

pub fn get_current_process_thread_nums() {
    if let Some(count) = num_threads::num_threads() {
        println!("num_threads: {}", count);
    }

    if let Some(count) = thread_amount::thread_amount() {
        println!("thread_amount: {}", count);
    }
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

pub fn thread_unpark_before_park() {
    let handle = thread::spawn(|| {
        thread::sleep(std::time::Duration::from_secs(1));
        thread::park();
        println!("resume");
        thread::park();
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });

    // change park token many time
    handle.thread().unpark();
    // the following unpark will not be effect, because token is not PARKED
    // the unpark source code can show that
    handle.thread().unpark();
    handle.thread().unpark();

    handle.join().unwrap();
}
