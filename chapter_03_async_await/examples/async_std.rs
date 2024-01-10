fn main() {
    async_std::task::block_on(hello_async());
}

async fn hello_async() {
    println!("Hello, async world!");
}
