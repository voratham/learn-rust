use std::time::Duration;

async fn do_work() {
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn timeout(seconds: f32) {
    tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_work() => println!("do_work finished first"),
        _ = timeout(3.0) => println!("Timeout finished first")
    }
}
