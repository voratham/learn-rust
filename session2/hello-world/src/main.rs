fn hello_thread(n: u32) {
    println!("Hello from thread {n}");
}

fn do_math(i: u32) -> u32 {
    let mut n = i + 1;
    for _ in 0..10 {
        n *= 2;
    }
    n
}

fn main() {
    println!("Hello with from the main thread");

    let mut thread_handles = Vec::new();
    for i in 0..10 {
        // let thread_handle = std::thread::spawn(move || hello_thread(i));
        let thread_handle = std::thread::spawn(move || do_math(i));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|h| {
        println!("{}", h.join().unwrap());
    });
    // thread_handles.into_iter().for_each(|h| h.join().unwrap());

    // Basic spawn thread
    // let thread_handle = std::thread::spawn(hello_thread);
    // thread_handle.join().unwrap();
}
