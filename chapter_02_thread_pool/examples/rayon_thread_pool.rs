use chapter_02_thread_pool::*;

fn main() {
    rayon_thread_pool_fib();
    rayon_scoped_thread_pool();
}

fn rayon_thread_pool() -> rayon::ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(4)
        .thread_name(|i| format!("worker-{}", i))
        .build()
        .unwrap()
}

fn rayon_thread_pool_fib() {
    let pool = rayon_thread_pool();
    let n = pool.install(|| fib(20));
    println!("{}", n);
}

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2));
    return a + b;
}

scoped_tls::scoped_thread_local!(static POOL_DATA: Vec<i32>);
fn rayon_scoped_thread_pool() {
    let pool_data = vec![1, 2, 3];

    assert!(!POOL_DATA.is_set());

    ThreadPoolBuilder::new()
        .build_scoped(
            |thread| POOL_DATA.set(&pool_data, || thread.run()),
            |pool| pool.install(|| assert!(POOL_DATA.is_set())),
        )
        .unwrap();

    drop(pool_data);
}
