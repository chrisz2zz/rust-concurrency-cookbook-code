#![feature(lazy_cell)]
use std::{
    borrow::Borrow,
    cell::{Cell, LazyCell, OnceCell, RefCell, RefMut},
    collections::HashMap,
    ops::Deref,
    rc::Rc,
    sync::LazyLock,
};

fn main() {
    cell();
    refcell();
    oncecell();
    lazycell();
    lazylock();
}

fn cell() {
    let x = Cell::new(42);
    let y = &x;

    x.set(10);

    println!("y: {:?}", y.get());
    println!("y: {:?}", y.get());
}

fn refcell() {
    let x = RefCell::new(42);
    {
        let y = x.borrow();
        println!("y: {:?}", *y.borrow());
    }

    {
        let mut z = x.borrow_mut();
        *z = 10;
    }

    println!("x: {:?}", x.borrow().deref());
}

fn oncecell() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value: &String = cell.get_or_init(|| "Hello World!".to_string());
    assert_eq!(value, "Hello World!");
    assert!(cell.get().is_some())
}

fn lazycell() {
    let lazy = LazyCell::new(|| {
        println!("initializing");
        46
    });

    println!("ready");
    println!("{}", *lazy);
    println!("{}", *lazy);
}

static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    m
});

fn lazylock() {
    println!("ready");
    std::thread::spawn(|| {
        println!("{:?}", HASHMAP.get(&13));
    })
    .join()
    .unwrap();
    println!("{:?}", HASHMAP.get(&74));
}

fn rc() {
    let data = Rc::new(42);

    let reference1 = Rc::clone(&data);
    let reference2 = Rc::clone(&data);

    let ss = String::from("12333");
    let ss1 = &ss;
    let ss2 = &ss;

    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    let total: i32 = RefCell::borrow(&shared_map).values().sum();
    println!("{total}");
}
