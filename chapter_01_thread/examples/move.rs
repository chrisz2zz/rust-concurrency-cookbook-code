use chapter_01_thread::*;

fn main() {
    start_one_thread_with_move();
    start_one_thread_with_move2();
}

pub fn start_one_thread_with_move() {
    let x = 100;

    let handle1 = thread::spawn(move || {
        println!("Hello from a thread with move, x={}!", x);
    });

    // handle.join().unwrap();

    // i32 implement Copy trait, move just like copy x to thread
    let handle2 = thread::spawn(move || {
        println!("Hello from a thread with move again, x={}!", x);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let handle = thread::spawn(|| {
        println!("Hello from a thread without move");
    });

    handle.join().unwrap();
}

pub fn start_one_thread_with_move2() {
    let x = vec![1, 2, 3];
    let x_clone = x.clone();
    let handle1 = thread::spawn(move || {
        println!("Hello from a thread with move, x={:?}", x);
    });

    // the x`s ownership move to last thread, next thread can not use x
    // let handle2 = thread::spawn(move || {
    //     println!("Hello from a thread with move, x={:?}", x);
    // });

    // otherwise clone x before move and use x_clone in next thread
    let handle2 = thread::spawn(move || {
        println!("Hello from a thread with move, x={:?}", x_clone);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let handle = thread::spawn(|| {
        println!("Hello from a thread without move");
    });

    handle.join().unwrap();
}
