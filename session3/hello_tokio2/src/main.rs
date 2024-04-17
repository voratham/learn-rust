async fn hello() -> u32 {
    println!("Hello Tokio");
    3
}

async fn hello2() -> u32 {
    println!("Hello Tokio 2");
    4
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // hello().await;

    // let result = tokio::join!(hello(), hello2());
    // println!("{result:?}");
    // let (one, two) = result;

    /*
       tokio::spawn(ticker());
       hello().await;
    */

    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker())
    );
    println!("Finished")
}
