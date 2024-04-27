use std::rc::Rc;

#[derive(Debug)]
struct Droppable(i32);

impl Droppable {
    fn new(n: i32) -> Self {
        println!("DroppableStruct Constructing {n}");
        Self(n)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0)
    }
}

fn move_me(x: Rc<Droppable>) {
    println!("Moved {}", x.0)
}

fn main() {
    println!("Hello, world!");
}
