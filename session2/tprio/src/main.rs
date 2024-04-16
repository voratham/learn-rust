use std::{sync::atomic::AtomicI32, time::Duration};

use thread_priority::*;

static LOW_COUNT: AtomicI32 = AtomicI32::new(0);
static MEDIUM_COUNT: AtomicI32 = AtomicI32::new(0);
static HIGH_COUNT: AtomicI32 = AtomicI32::new(0);

fn low_priority() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop {
        LOW_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn regular_priority() {
    set_current_thread_priority(ThreadPriority::Max).unwrap();
    loop {
        MEDIUM_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn main() {
    std::thread::spawn(low_priority);
    std::thread::spawn(regular_priority);
    std::thread::sleep(Duration::from_secs(5));
    println!(
        "Low: {}",
        LOW_COUNT.load(std::sync::atomic::Ordering::Relaxed)
    );

    println!(
        "Medium: {}",
        LOW_COUNT.load(std::sync::atomic::Ordering::Relaxed)
    );
}
