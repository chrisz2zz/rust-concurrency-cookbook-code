use std::sync::Once;

static INIT: Once = Once::new();

fn main() {
    example1();

    println!("{}", example2());
}

fn example1() {
    INIT.call_once(|| {
        println!("Initialization code executed!");
    });

    INIT.call_once(|| {
        println!("This won`t be printed.");
    });
}

static INIT2: Once = Once::new();
static mut GLOBAL_CONFIG: Option<String> = None;

fn example2() -> &'static str {
    fn init_global_config() {
        unsafe {
            GLOBAL_CONFIG = Some("fffff".to_string());
        }
    }

    INIT2.call_once(|| init_global_config());

    unsafe { GLOBAL_CONFIG.as_ref().unwrap() }
}
