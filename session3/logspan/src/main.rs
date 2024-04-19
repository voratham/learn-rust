use std::time::Duration;
use tracing_subscriber::fmt::format::FmtSpan;

// #[tracing::instrument]
// async fn hello_world() {
//     println!("Hello !");
//     tokio::time::sleep(Duration::from_secs(1)).await;
// }

#[tracing::instrument]
async fn hello_world() {
    println!("1 hello, world!");
}

#[tracing::instrument]
fn hello_world2() {
    println!("2 hello, world!");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let subscriber = tracing_subscriber::FmtSubscriber::new();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Starting up");
    tracing::warn!("Are you sure this is a good idea ?");
    tracing::error!("Something went horribly wrong");
    hello_world();
    hello_world2();

    Ok(())
}
