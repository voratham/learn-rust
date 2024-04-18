use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let result: io::Lines<BufReader<File>> = io::BufReader::new(file).lines();

    Ok(result)
}

// NOTE: using async
async fn async_line_count(filename: String) -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;

    println!("Reading {filename}...");
    let now = std::time::Instant::now();
    let mut line_count = 0;

    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }
    println!(
        "Read {} lines in {:.3} seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );
    Ok(line_count)
}

async fn line_count(filename: String) -> anyhow::Result<usize> {
    println!("Reading {filename}...");
    let now = std::time::Instant::now();
    let mut line_count = 0;
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        })
    }

    println!(
        "Read {} lines in {:.3} seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );
    Ok(line_count)
}

#[tokio::main] // NOTE: using multi core auto
async fn main() -> anyhow::Result<()> {
    let filename = String::from("warandpeace.txt");

    let now = std::time::Instant::now();
    let (c1, c2) = tokio::join!(
        line_count(filename.to_owned()),
        line_count(filename.to_owned())
    );
    println!("Sync Total lines: {}", c1? + c2?);
    println!("Sync In {:.3} seconds", now.elapsed().as_secs_f32());

    // Asynchronous version
    let now = std::time::Instant::now();
    let (c1, c2) = tokio::join!(
        async_line_count(filename.to_owned()),
        async_line_count(filename.to_owned())
    );
    println!("Total lines: {}", c1? + c2?);
    println!("Async In {:.3} seconds", now.elapsed().as_secs_f32());

    Ok(())
}
