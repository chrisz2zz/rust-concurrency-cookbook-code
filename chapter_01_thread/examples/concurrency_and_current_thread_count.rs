use chapter_01_thread::*;

fn main() {
    get_available_parallelism();
    get_cpu_cores();
    get_cpu_nums();
    get_current_process_thread_nums();
}

pub fn get_available_parallelism() {
    let count = thread::available_parallelism().unwrap().get();
    println!("available parallelism count: {}", count);
    assert!(count >= 1_usize);
}

pub fn get_cpu_cores() {
    println!(
        "cores: {:?}",
        (0..affinity::get_core_num()).collect::<Vec<usize>>()
    )
}

pub fn get_cpu_nums() {
    println!("cpu logic core nums: {}", num_cpus::get())
}

pub fn get_current_process_thread_nums() {
    if let Some(count) = num_threads::num_threads() {
        println!("num_threads: {}", count);
    }

    if let Some(count) = thread_amount::thread_amount() {
        println!("thread_amount: {}", count);
    }
}
