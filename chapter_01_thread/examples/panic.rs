use chapter_01_thread::*;
fn main() {
    panic_example();
    panic_caught_example();
}

pub fn panic_example() {
    println!("Hello, world!");
    let h = thread::spawn(|| {
        thread::sleep(time::Duration::from_secs(1));
        panic!("boom");
    });
    let r = h.join();
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e),
    }
    println!("Exiting main!");
}

pub fn panic_caught_example() {
    println!("Hello, panic_caught_example!");
    let h = thread::spawn(|| {
        thread::sleep(time::Duration::from_secs(1));
        let result = std::panic::catch_unwind(|| {
            panic!("boom");
        });
        println!("panic caught, result = {}", result.is_err());
    });

    let r = h.join();
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e),
    }

    println!("Exiting main!");
}
