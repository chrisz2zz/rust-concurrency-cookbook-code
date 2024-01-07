use std::ops::Deref;
use std::{rc::Rc, sync::mpsc::channel};

use chapter_01_thread::*;
use send_wrapper::SendWrapper;

fn main() {
    send_wrapper();
    send_wrapper_custom();
}

/*
pub fn wrong_send() {
    let counter = Rc::new(42);
    let (sender, receiver) = channel();

    // Rc not implement Send Trait
    let _t = thread::spawn(move || {
        sender.send(counter).unwrap();
    });

    let value = receiver.recv().unwrap();
    println!("received from the main thread: {}", value);
}
 */

pub fn send_wrapper() {
    let wrapped_value = SendWrapper::new(Rc::new(42));
    let (sender, receiver) = channel();

    let _t = thread::spawn(move || {
        sender.send(wrapped_value).unwrap();
    });

    let wrapped_value = receiver.recv().unwrap();

    let value = wrapped_value.deref();
    println!("received from the main thread: {}", value);
}

pub fn send_wrapper_custom() {
    let wrapped_value = A { data: Rc::new(42) };
    let (sender, receiver) = channel();

    let _t = thread::spawn(move || {
        sender.send(wrapped_value).unwrap();
    });

    let wrapped_value = receiver.recv().unwrap();

    // let value = wrapped_value.deref();
    println!("received from the main thread: {}", wrapped_value.data);
}

struct A<T> {
    data: T,
}

unsafe impl<T> Send for A<T> {}
