#![feature(exclusive_range_pattern)]

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use poolite::Pool;
fn main() {
    poolite_fib();
}

fn new() -> poolite::Pool {
    let pool = poolite::Pool::new().unwrap();
    pool.push(|| println!("hello"));
    pool
}

fn scoped() {
    let pool = new();
    pool.scoped(|s| {
        s.push(|| println!("hello"));
    })
}

fn new_with_builder() -> poolite::Pool {
    poolite::Builder::new().max(8).build().unwrap()
}

fn poolite_fib() {
    let pool = Pool::new().unwrap();
    let map = Arc::new(Mutex::new(BTreeMap::new()));
    for i in 0..10 {
        let map = map.clone();
        pool.push(move || test(i, map));
    }

    pool.join();

    for (k, v) in map.lock().unwrap().iter() {
        println!("key: {}\tvalue: {}", k, v);
    }
}

fn test(msg: i32, map: Arc<Mutex<BTreeMap<i32, i32>>>) {
    let res = fib(msg);
    let mut maplock = map.lock().unwrap();
    maplock.insert(msg, res);
}

fn fib(msg: i32) -> i32 {
    match msg {
        0..2 => 1,
        x => fib(x - 1) + fib(x - 2),
    }
}
