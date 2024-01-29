use std::{
    sync::mpsc::{channel, sync_channel},
    thread,
};

fn main() {
    sample();
    multi_producer();
    sync_channel_example();
    err_example();
}

fn sample() {
    let (tx, rx) = channel();

    thread::spawn(move || tx.send(10));

    assert_eq!(rx.recv().unwrap(), 10);
}

fn multi_producer() {
    let (tx, rx) = channel();

    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(i));
    }

    for _ in 0..10 {
        println!("{}", rx.recv().unwrap());
    }
}

fn sync_channel_example() {
    let (tx, rx) = sync_channel(3);

    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(i));
    }

    drop(tx);

    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }
}

fn err_example() {
    let (tx, rx) = channel::<isize>();
    drop(tx);
    println!("{:?}", rx.recv());
}
