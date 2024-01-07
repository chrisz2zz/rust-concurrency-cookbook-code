#![feature(thread_spawn_unchecked)]

use chapter_01_thread::*;

fn main() {
    start_scoped_threads();
    unsafe_start_threads_without_scoped();
}

pub fn start_scoped_threads() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            x += a[0] + a[2];
        });
    });

    a.push(4);
    assert_eq!(x, a.len());
}

/*
pub fn wrong_start_threads_without_scoped() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::spawn(move || {
        println!("hello from the first scoped thread");
        dbg!(&a);
    });

    // use of moved value: `a` value used here after move
    thread::spawn(move || {
        println!("hello from the second scoped thread");
        x += a[0] + a[2];
    });

    a.push(4);
    assert_eq!(x, a.len());
}
 */

pub fn unsafe_start_threads_without_scoped() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    unsafe {
        thread::Builder::new()
            .spawn_unchecked(|| {
                println!("hello from the first scoped thread");
                dbg!(&a);
            })
            .unwrap()
            .join()
            .unwrap();
    };

    unsafe {
        // use of moved value: `a` value used here after move
        thread::Builder::new()
            .spawn_unchecked(|| {
                println!("hello from the second scoped thread");
                x += a[0] + a[2];
            })
            .unwrap()
            .join()
            .unwrap();
    };

    println!("hello from main thread");
    a.push(4);
    assert_eq!(x, a.len());
}
