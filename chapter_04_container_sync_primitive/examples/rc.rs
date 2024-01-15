use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

fn main() {
    rc();
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

    let total: i32 = shared_map.borrow().values().sum();
    println!("{total}");
}
