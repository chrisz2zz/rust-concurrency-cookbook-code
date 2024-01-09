use fast_threadpool::ThreadPoolConfig;

fn main() {
    fast_threadpool_example().unwrap();
    fast_threadpool_with_tokio();
}

fn fast_threadpool_example() -> Result<(), fast_threadpool::ThreadPoolDisconnected> {
    let threadpool =
        fast_threadpool::ThreadPool::start(ThreadPoolConfig::default(), ()).into_sync_handler();

    assert_eq!(4, threadpool.execute(|_| 2 + 2)?);

    Ok(())
}

fn fast_threadpool_with_tokio() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let threadpool = fast_threadpool::ThreadPool::start(ThreadPoolConfig::default(), ())
            .into_async_handler();
        assert_eq!(4, threadpool.execute(|_| { 2 + 2 }).await.unwrap());
    })
}
