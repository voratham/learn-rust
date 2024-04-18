use std::{sync::mpsc, time::Duration};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel::<Command>();

    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);

    let handle = tokio::runtime::Handle::current();

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => {
                    let tx_reply = tx_reply.clone();
                    handle.spawn(async move {
                        tx_reply.send(s).await.unwrap();
                    });
                    // println!("{s}")
                }
            }
        }
    });

    // Receive message
    tokio::spawn(async move {
        while let Some(reply) = rx_reply.recv().await {
            println!("{reply}");
        }
    });

    //  Launch the async sender
    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }

    // let (tx_broadcast, mut rx_broadcast) = tokio::sync::broadcast::channel::<String>(16);

    // tx_broadcast.send("hello".to_string()).unwrap();
}

// #[tokio::main]
// async fn main() {
//     let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(16);

//     for n in 0..16 {
//         let mut messages = tx.subscribe();
//         tokio::spawn(async move {
//             while let Ok(msg) = messages.recv().await {
//                 println!("{n}: {msg}");
//             }
//         });
//     }

//     tx.send("xx-hello".to_string()).unwrap();
//     while let Ok(msg) = rx.recv().await {
//         println!("main: {msg}");
//     }
// }
