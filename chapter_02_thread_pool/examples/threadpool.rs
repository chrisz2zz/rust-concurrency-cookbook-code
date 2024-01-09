use std::sync::{atomic::AtomicUsize, mpsc::channel, Arc, Barrier};

use chapter_02_thread_pool::*;
fn main() {
    threadpool();
    threadpool_with_barrier();
}

fn threadpool() {
    let pool = threadpool::ThreadPool::new(4);

    let (sender, receiver) = channel();
    for i in 0..8 {
        let sender = sender.clone();
        pool.execute(move || {
            let result = i * 2;
            sender.send(result).expect("发送失败");
        });
    }

    for _ in 0..8 {
        let result = receiver.recv().expect("接受失败");
        println!("任务结果: {}", result);
    }
}

fn threadpool_with_barrier() {
    let n_works = 42;
    let n_jobs = 23;
    let pool = threadpool::ThreadPool::new(n_works);
    let an_atomic = Arc::new(AtomicUsize::new(0));

    assert!(n_jobs <= n_works, "too many jobs, will deadlock");

    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    for _ in 0..n_jobs {
        let barrier = barrier.clone();
        let an_atomic = an_atomic.clone();

        pool.execute(move || {
            an_atomic.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            // 等待线程完成
            barrier.wait();
        });
    }

    barrier.wait();
    assert_eq!(an_atomic.load(std::sync::atomic::Ordering::SeqCst), 23);
}
