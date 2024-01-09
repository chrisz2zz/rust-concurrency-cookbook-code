fn main() {
    scoped_threadpool();
}

fn scoped_threadpool() {
    let mut pool = scoped_threadpool::Pool::new(4);
    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];

    pool.scoped(|s| {
        for e in &mut vec {
            s.execute(move || {
                *e += 1;
            });
        }
    });

    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
