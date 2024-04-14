fn double(n: i32) -> i32 {
    n * 2
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        n * 2
    } else {
        0
    }
}

fn greet(s: String) -> String {
    println!("Hello {s}");
    s
}

fn greet_borrow(s: &String) {
    println!("[greet_borrow] {s}")
}

fn greet_borrow_mut(s: &mut String) {
    *s = format!("[greet_borrow_mut] Hello {s}")
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

fn main() {
    // let n: i32 = double(5);
    // println!("{}", double(5));
    // let n : i32 = {
    //      5 * 12
    // };
    // println!("{}", n)

    // let i: i32 = 5;
    // let n: i32 = if i == 5 { 6 } else { 7 };
    // println!("{}", n)

    // let name: String = "Hello".to_string();
    // let name = greet(name);
    // greet(name);

    // let name: String = "Hello".to_string();
    // greet(name.clone());
    // greet(name);

    // let mut name: String = "Hello".to_string();
    // // greet(name);
    // greet_borrow(&name);
    // // greet_borrow_mut(&mut name);
    // println!("{name}");

    let input = read_line();
    println!("You typed: [{input}]");
}
