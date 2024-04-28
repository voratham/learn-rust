use std::sync::Arc;

#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("Droppable constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("dropping {} ", self.0);
    }
}

fn move_move(x: Arc<Droppable>, index: i32) {
    println!("moved index{} - x{}", index, x.0);
}

fn main() {
    let my_shared = Arc::new(Droppable::new(1));

    {
        let _x1 = my_shared.clone();
        let _x2 = my_shared.clone();
        let _x3 = my_shared.clone();
    }

    move_move(my_shared.clone(), 999);

    let mut threads = Vec::new();
    for index in 0..10 {
        let my_shared = my_shared.clone();
        threads.push(std::thread::spawn(move || move_move(my_shared, index)))
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("my_shared : {my_shared:?}");
    println!("finished...")
}
