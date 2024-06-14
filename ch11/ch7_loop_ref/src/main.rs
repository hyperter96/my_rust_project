use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("1 a rc count: {}", Rc::strong_count(&a)); // 1 a rc count: 1
    println!("1 a tail: {:?}", a.tail()); // 1 a tail: Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("2 a rc count: {}", Rc::strong_count(&a)); // 2 a rc count: 2
    println!("2 b rc count: {}", Rc::strong_count(&b)); // 2 b rc count: 1
    println!("2 a tail: {:?}", b.tail()); // 2 a tail: Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("3 a rc count: {}", Rc::strong_count(&a)); // 3 a rc count: 2
    println!("3 b rc count: {}", Rc::strong_count(&b)); // 3 b rc count: 2
}
