fn main() {
    rayon_scope();
}

pub fn rayon_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    rayon::scope(|s| {
        s.spawn(|_| {
            println!("hello from the first rayon scoped thread");
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
    });

    a.push(4);
    assert_eq!(x, a.len());
}
