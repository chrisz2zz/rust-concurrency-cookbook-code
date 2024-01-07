use chapter_01_thread::*;

fn main() {
    start_thread_with_priority();
    thread_builder();
    get_priority();
}

pub fn start_thread_with_priority() {
    let handle1 = thread::spawn(|| {
        assert!(set_current_thread_priority(thread_priority::ThreadPriority::Min).is_ok());
        println!("Hello from a thread5!");
    });

    let handle2 = thread::spawn(|| {
        assert!(set_current_thread_priority(thread_priority::ThreadPriority::Max).is_ok());
        println!("Hello from a thread6!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn thread_builder() {
    let thread1 = thread_priority::ThreadBuilder::default()
        .name("MyThread")
        .priority(thread_priority::ThreadPriority::Max)
        .spawn(|result| {
            println!("Set priority result: {:?}", result);
            assert!(result.is_ok());
        })
        .unwrap();

    let thread2 = thread_priority::ThreadBuilder::default()
        .name("MyThread")
        .priority(thread_priority::ThreadPriority::Max)
        .spawn_careless(|| {
            println!("We don`t care about the priority result.");
        })
        .unwrap();

    thread1.join().unwrap();
    thread2.join().unwrap();
}

pub fn get_priority() {
    println!(
        "current thread priority: {:?}",
        thread::current().get_priority().unwrap()
    );

    println!(
        "current thread native id is {:?}",
        thread::current().get_native_id()
    );
}
