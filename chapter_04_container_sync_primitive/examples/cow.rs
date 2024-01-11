use beef;
use std::{borrow::Cow, mem::size_of};

fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1; // 共享统一份内存, 但内存所有权也move到s2了

    s2.push_str(" world"); // 开辟新内存  复制旧内存 追加数据
                           // println!("{}", s1); // move到s2了

    let origin = "hello world";
    let mut cow = Cow::from(origin);
    assert_eq!(cow, "hello world");

    let s: &str = &cow;
    assert_eq!(s, "hello world");

    assert_eq!(s.len(), cow.len());

    // let s: String = cow.into();
    // assert_eq!(s, "HELLO WORLD");

    let s: &mut str = cow.to_mut();
    s.make_ascii_uppercase();
    assert_eq!(s, "HELLO WORLD");
    assert_eq!(origin, "hello world");

    beef_cow();
}

fn beef_cow() {
    let borrowed: beef::Cow<str> = beef::Cow::borrowed("Hello");
    let owned: beef::Cow<str> = beef::Cow::owned(String::from("World"));
    let _ = beef::Cow::from("Hello");

    assert_eq!(format!("{} {}!", borrowed, owned), "Hello World!");

    const WORD: usize = size_of::<usize>();

    assert_eq!(size_of::<std::borrow::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
    assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);
}
