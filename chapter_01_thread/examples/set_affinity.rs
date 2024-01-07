fn main() {
    use_affinity();
}

#[cfg(not(target_os = "macos"))]
pub fn use_affinity() {
    let cores: Vec<usize> = (0..affinity::get_core_num()).step_by(2).collect();
    println!("Binding thread to cores: {:?}", cores);

    affinity::set_thread_affinity(&cores).unwrap();
    println!(
        "Current thread affinity: {:?}",
        affinity::get_thread_affinity().unwrap()
    )
}
