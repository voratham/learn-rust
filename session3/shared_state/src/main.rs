use std::sync::Mutex;

use once_cell::sync::Lazy;

static COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));
// static COUNTER: Mutex<u32> = Mutex::new(0);

// async fn increment() {
//     let mut counter = COUNTER.lock().unwrap();
//     *counter += 1;
// }

async fn increment() {
    let mut counter = COUNTER.lock().unwrap();
    *counter = add_one(*counter).await;
}

async fn add_one(n: u32) -> u32 {
    n + 1
}

#[tokio::main]
async fn main() {
    println!("start program...");

    tokio::join!(increment(), increment(), increment());
    println!("COUNTER = {}", COUNTER.lock().unwrap())
}
