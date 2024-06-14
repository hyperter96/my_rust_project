use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

trait Animal {
    fn eat(&self);
}

#[derive(Debug)]
struct Cat {
    children: Option<Box<Cat>>,
}

impl Animal for Cat {
    fn eat(&self) {
        println!("cat is eating");
    }
}
fn main() {
    let b = Box::new(5); // b存储在栈上，5存储在堆上，b指向5所在的内存
    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list); // Cons(1, Cons(2, Cons(3, Nil)))

    let cat = Box::new(Cat{ children: None });
    println!("{:?}", cat); // Cat { children: None }
    let t: Box<dyn Animal>;
    t = Box::new(Cat {
        children: Some(cat),
    });
    t.eat(); // cat is eating
}
