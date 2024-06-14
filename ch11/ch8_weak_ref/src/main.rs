use std::{cell::RefCell, rc::{Rc, Weak}};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!("1, a strong count: {}, weak count: {}", Rc::strong_count(&a), Rc::weak_count(&a)); // 1, a strong count: 1, weak count: 0
    println!("1, a tail: {:?}", a.tail()); // 1, a tail: Some(RefCell { value: (Weak) })

    let b = Rc::new(Cons(10, RefCell::new(Weak::new())));
    if let Some(link) = b.tail() {
        *link.borrow_mut() = Rc::downgrade(&a);
    }

    println!("2, a strong count: {}, weak count: {}", Rc::strong_count(&a), Rc::weak_count(&a)); // 2, a strong count: 1, weak count: 1
    println!("2, b strong count: {}, weak count: {}", Rc::strong_count(&b), Rc::weak_count(&b)); // 2, b strong count: 1, weak count: 0
    println!("2, b tail: {:?}", b.tail()); // 2, b tail: Some(RefCell { value: (Weak) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }

    println!("3, a strong count: {}, weak count: {}", Rc::strong_count(&a), Rc::weak_count(&a)); // 3, a strong count: 1, weak count: 1
    println!("3, b strong count: {}, weak count: {}", Rc::strong_count(&b), Rc::weak_count(&b)); // 3, b strong count: 1, weak count: 1
    println!("3, a tail: {:?}", a.tail()); // 3, a tail: Some(RefCell { value: (Weak) })
}
