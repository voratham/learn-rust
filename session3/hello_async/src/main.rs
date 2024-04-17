use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
// NOTE: if we want using async await function we will install futures by recommended from rust docs.

fn do_something_sync() {
    print!("Not async !!");
}

// Define async function
async fn say_hello() {
    println!("Hello Async From SayHello");
    // second_function().await
    join!(second_function(), say_good_bye());

    let n = double(4).await;
    println!("{n}");

    let futures = vec![double(1), double(2), double(3)];
    let results = join_all(futures).await;
    println!("{results:?}");

    do_something_sync();
}

async fn second_function() {
    println!("Hello again...");
}

async fn say_good_bye() {
    println!("GoodBye 👋")
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    // NOTE: we cannot using async await without futures library
    // let future = say_hello();
    // future.await;
    block_on(say_hello())
}
