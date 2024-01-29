use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Failed to execute cmd");
    println!("{:?}", output);

    let mut child = Command::new("ls")
        .arg("-l")
        .spawn()
        .expect("Failed to execute cmd");

    println!("{:?}", child.wait().expect("Failed to wait for cmd"));

    println!(
        "{:?}",
        Command::new("echo")
            .arg("Hello Rust!")
            .stdout(Stdio::piped())
            .output()
            .unwrap()
    )
}
