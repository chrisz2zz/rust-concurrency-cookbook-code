use chapter_01_thread::*;

fn main() {
    control_thread();
}

pub fn control_thread() {
    let (flag, control) = make_pair();
    let handle = thread::spawn(move || {
        while flag.alive() {
            thread::sleep(time::Duration::from_millis(100));
            println!("I`m alive!");
        }
    });

    thread::sleep(time::Duration::from_millis(100));
    assert_eq!(control.is_done(), false);
    control.stop();
    handle.join().unwrap();

    assert_eq!(control.is_interrupted(), false);
    assert_eq!(control.is_done(), true);

    println!("This thread is stopped");
}
