fn main() {
    futures_lite::future::block_on(hello_async());
}

async fn hello_async() {
    println!("Hello, async world!");
}
