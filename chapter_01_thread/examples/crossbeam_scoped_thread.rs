fn main() {
    crossbeam_scope();
}

pub fn crossbeam_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    crossbeam::thread::scope(|s| {
        s.spawn(|_| {
            println!("hello from the first crossbeam scoped thread");
            dbg!(&a);
        });

        s.spawn(|ss| {
            println!("hello from the second crossbeam scoped thread");
            x += a[0] + a[2];
            // can create scoped thread in scoped thread
            ss.spawn(|_| {
                println!("hello from the first crossbeam scoped thread in second crossbeam scoped thread");
            });
        });

        println!("hello from the main thread");
    })
    .unwrap();

    a.push(4);
    assert_eq!(x, a.len());
}
