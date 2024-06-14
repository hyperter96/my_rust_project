use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a)); // count after creating a: 1
    let b = Cons(3, Rc::clone(&a));
    println!("count after binding to b, a count: {}", Rc::strong_count(&a)); // count after binding to b, a count: 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after binding to c, a count: {}", Rc::strong_count(&a)); // count after binding to c, a count: 3
    }
    println!("{:?}", b);
    println!("count at end a: {}", Rc::strong_count(&a)); // count at end a: 2 由于c离开作用域
    let d = Cons(6, a.clone()); // 另一种写法
    let e = Cons(7, a.clone());
}
