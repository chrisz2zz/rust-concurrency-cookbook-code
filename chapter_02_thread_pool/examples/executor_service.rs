use std::sync::{atomic::AtomicUsize, Arc};
use std::thread;
use std::time::Duration;

use executor_service::Executors;

fn main() {
    execute();
    example();
}

fn execute() {
    // let pool = Executors::new_fixed_thread_pool(4).unwrap();
    let mut pool = Executors::new_cached_thread_pool(None).unwrap();

    pool.execute(|| println!("hello")).unwrap();
}

fn example() {
    let mut executor_service =
        Executors::new_fixed_thread_pool(10).expect("Failed to create the thread pool");

    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..10 {
        let counter = counter.clone();
        executor_service
            .execute(move || {
                thread::sleep(Duration::from_millis(100));
                counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            })
            .unwrap();
    }

    thread::sleep(Duration::from_secs(1));
    assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 10);
    let mut executor_service =
        Executors::new_fixed_thread_pool(2).expect("failed to create the thread pool");

    let some_param = "Mr White";
    let res = executor_service
        .submit_sync(move || {
            thread::sleep(Duration::from_secs(5));
            println!("Hello {:?}", some_param);
            println!("Long computation finished");
            2
        })
        .expect("failed to submit function");

    println!("Result: {:#?}", res);
    assert_eq!(res, 2);
}
