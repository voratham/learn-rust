use tokio::runtime;

async fn hello() {
    println!("hello Tokio")
}

// fn main() {
//     // NOTE: use just single-thread
//     // let rt = runtime::Builder::new_current_thread()
//     //     .enable_all()
//     //     .build()
//     //     .unwrap();

//     // NOTE: use just multi-thread
//     let rt = runtime::Builder::new_multi_thread()
//         .enable_all()
//         .worker_threads(4)
//         .build()
//         .unwrap();

//     rt.block_on(hello());

// }

#[tokio::main] // NOTE: in case multi-thread
// #[tokio::main(flavor = "current_thread")] // NOTE: in case single thread
async fn main() {
    hello().await;
}
