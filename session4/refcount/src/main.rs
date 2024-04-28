use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

struct SharedData(String);

struct SharedDataInternal {
    data: Mutex<String>,
}

impl SharedDataInternal {
    fn new(s: &str) -> Self {
        Self {
            data: Mutex::new(s.to_string()),
        }
    }
}

fn main() {
    // let my_shared = Arc::new(Mutex::new(SharedData("Hello".to_string())));

    // let mut threads = Vec::new();
    // for i in 0..10 {
    //     let my_shared = my_shared.clone();
    //     threads.push(std::thread::spawn(move || {
    //         // sleep(Duration::from_secs(5));
    //         let mut data = my_shared.lock().unwrap();
    //         data.0.push_str(&format!(" {i}"));
    //     }))
    // }

    // Internal mutex pattern
    let my_shared = Arc::new(SharedDataInternal::new("HelloInternal"));

    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            // sleep(Duration::from_secs(5));

            let mut data = my_shared.data.lock().unwrap();
            data.push_str(&format!(" {i}"));
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    // let data = my_shared.lock().unwrap();
    let data = my_shared.data.lock().unwrap();
    println!("{data}")
}
