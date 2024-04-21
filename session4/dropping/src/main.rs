use core::str;

struct MyStruct {
    n: i32,
}

impl MyStruct {
    fn new(n: i32) -> Self {
        println!("MyStruct constructing {n}");
        Self { n }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("MyStruct {} Dropping", self.n)
    }
}

struct HasDroppables {
    x: MyStruct,
}

fn move_me(x: MyStruct) {
    println!("Moved {}", x.n);
}

fn main() {
    println!("start program");
    let x = MyStruct::new(1);
    {
        let y = MyStruct::new(2);
    }
    println!("Ended scope");
    move_me(x);
    println!("Back from function");

    let has_drop = HasDroppables {
        x: MyStruct::new(4),
    };

    println!("Ending the main 🔴")
}
