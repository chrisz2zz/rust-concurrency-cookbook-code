#![feature(thread_spawn_unchecked)]

use chapter_01_thread::*;

fn main() {
    start_one_thread_by_builder();
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
