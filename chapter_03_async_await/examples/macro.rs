use std::fmt::{Debug, Display};

use futures::{future::FutureExt, join, pin_mut, select, try_join};

fn main() {
    futures::executor::block_on(select_ex());
    futures::executor::block_on(join_ex());
    futures::executor::block_on(try_join_ex());
    smol_zip();
}

async fn select_ex() {
    let future1 = async {
        println!("future1");
    }
    .fuse();

    let future2 = async {
        println!("future2");
    }
    .fuse();

    pin_mut!(future1, future2);

    let result = select! {
        _ = future1 => 1,
        _ = future2 => 2,
    };

    println!("{}", result);
}

async fn join_ex() {
    let future1 = async {
        println!("future1");
    };

    let future2 = async {
        println!("future2");
    };

    let (_, _) = join!(future1, future2);
}

async fn try_join_ex() {
    let future1 = async { Ok::<i32, AAB>(1) };

    let future2 = async { Err::<i32, AAB>(AAB) };

    let result = try_join!(future1, future2);
    assert!(result.is_err());
}

fn smol_zip() {
    smol::block_on(async {
        use smol::future::{try_zip, zip};
        let future1 = async { 1 };
        let future2 = async { 2 };

        let result = zip(future1, future2);

        println!("smol_zip: {:?}", result.await);

        let future1 = async { Ok::<i32, AAB>(1) };
        let future2 = async { Err::<i32, AAB>(AAB) };

        let result = try_zip(future1, future2);
        assert!(result.await.is_err());
    })
}

struct AAB;

impl Display for AAB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AAB")
    }
}

impl Debug for AAB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AAB")
    }
}
