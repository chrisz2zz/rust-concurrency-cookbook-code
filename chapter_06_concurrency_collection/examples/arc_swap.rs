use std::sync::Arc;

use arc_swap::ArcSwap;

fn main() {
    let a = ArcSwap::new(Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    println!("{:?}", a.load());
    a.store(Arc::new(vec![999]));
    println!("{:?}", a.load());
}
