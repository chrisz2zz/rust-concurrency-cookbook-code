use std::sync::{atomic::AtomicI64, Arc};

use go_spawn::{go, join};

fn main() {
    go_thread();
}

pub fn go_thread() {
    let counter = Arc::new(AtomicI64::new(0));
    let counter_cloned = counter.clone();

    go! {
        for _ in 0..100 {
            counter_cloned.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    };

    assert!(join!().is_ok());
    assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 100);
}
