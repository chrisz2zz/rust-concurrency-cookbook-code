use std::thread;
use std::time::Duration;

fn main() {
    example1();
    example2();
}

fn example1() {
    let pool = threadpool_executor::threadpool::Builder::new()
        .core_pool_size(4)
        .maximum_pool_size(8)
        .keep_alive_time(Duration::from_secs(30))
        .build();

    pool.execute(|| println!("hello")).unwrap();

    let mut result: threadpool_executor::Expectation<Result<i32, ()>> = pool
        .execute(|| {
            thread::sleep(Duration::from_secs(5));
            Ok(1 + 2)
        })
        .unwrap();

    let res = result.get_result_timeout(Duration::from_secs(3));

    assert!(res.is_err());

    if let Err(e) = res {
        assert!(matches!(
            e.kind(),
            threadpool_executor::error::ErrorKind::TimeOut
        ));
    }

    let mut task = pool.execute(|| println!("hello")).unwrap();
    task.cancel().unwrap();
}

fn example2() {
    let pool = threadpool_executor::ThreadPool::new(1);
    let mut expectation = pool.execute(|| "hello, thread pool!").unwrap();
    assert_eq!(expectation.get_result().unwrap(), "hello, thread pool!");

    let pool = threadpool_executor::threadpool::Builder::new()
        .core_pool_size(1)
        .maximum_pool_size(3)
        .keep_alive_time(Duration::from_secs(300))
        .exeed_limit_policy(threadpool_executor::threadpool::ExceedLimitPolicy::Wait)
        .build();

    pool.execute(|| {
        thread::sleep(Duration::from_secs(3));
    })
    .unwrap();
    let mut exp = pool.execute(|| {}).unwrap();
    exp.cancel().unwrap();
}
