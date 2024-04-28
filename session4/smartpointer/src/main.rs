use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]

// Box type help about recursive
// Box will estimate size on rust-runtime
enum List {
    Node(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
    println!(" {:?} ", list);

    let x: i32 = 10;
    let y = Box::new(x);

    println!("{}", y);
    //  in case reference b,c point reference to a
    // let a = Box::new(10);
    // let b = Box::new(a);
    // let c = Box::new(a);

    // using Rc จะอนุญาติให้ หลาย ๆ ตัวแปรมี ownership ตัวเดียวกันได้กัน แต่ RefCell เป็น single ownership

    let a = Rc::new(Box::new(10));
    let b = Rc::clone(&a);
    let mut c = Rc::clone(&a);

    print!("Rc result b {:p} , c {:p} ", b, c);

    println!("in case try to change value from c variable");

    c = Rc::new(Box::new(20));
    println!("a {:p}", a);
    println!("c {:p}", c);

    //  มาลองใช้ RefCell
    let v1 = Rc::new(RefCell::new(10));
    let v2 = Rc::clone(&v1);
    let v3 = Rc::clone(&v1);

    println!("Before changed value v1 {:?}", v1);
    println!("before changed value v2 {:?}", v2);
    println!("before changed value v3 {:?}", v3);

    *v2.borrow_mut() = Box::new(20); // changed value v2
                                     // v1 , v3 it should see value equal 20

    println!("after changed value v1 {:?}", v1);
    println!("after changed value v2 {:?}", v2);
    println!("after changed value v3 {:?}", v3);
}
