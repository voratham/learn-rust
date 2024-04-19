// NOTE : sync version
// fn fibonacci(n: u32) -> u32 {
//     match n {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }

// fn main() {
//     println!("fibonacci(10) = {}", fibonacci(10))
// }

// use async_recursion::*;

// use futures::future::{BoxFuture, FutureExt};
// NOTE: async version
// fn fibonacci(n: u32) -> BoxFuture<'static, u32> {
//     async move {
//         match n {
//             0 => 0,
//             1 => 1,
//             _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
//         }
//     }
//     .boxed()
// }

use std::pin::Pin;

// NOTE: async using with library async_recursion
use async_recursion::async_recursion;
use futures::Future;

#[async_recursion]
async fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

async fn one() {
    println!("One");
}

async fn two() {
    println!("Two");
}

async fn call_one_of_them(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("Invalid choice"),
    }
}

#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", fibonacci(10).await);

    let future = async {
        println!("Hello, world!");
    };
    // if not pin complier will error for using Box::pin
    tokio::pin!(future);

    (&mut future).await; //&mut borrow not owner

    let boxed_future = call_one_of_them(1).await;
    boxed_future.await;
}
