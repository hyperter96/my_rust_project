use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
fn main() {
    let val = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    println!("a before {:?}", a); // a before Cons(RefCell { value: 5 }, Nil)
    println!("b before {:?}", b); // b before Cons(RefCell { value: 6 }, Cons(RefCell { value: 5 }, Nil))
    println!("c before {:?}", c); // c before Cons(RefCell { value: 7 }, Cons(RefCell { value: 5 }, Nil))

    *val.borrow_mut() += 10; // 修改内部的RefCell，Rc是只读的不可变引用，只提供数据共享
    println!("a after {:?}", a); // a after Cons(RefCell { value: 15 }, Nil)
    println!("b after {:?}", b); // b after Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after {:?}", c); // c after Cons(RefCell { value: 7 }, Cons(RefCell { value: 15 }, Nil))

    // &str是copy, String不copy
    let c = Cell::new("yzzy");
    let c1 = c.get();
    println!("{c1}");

    c.set("原子");
    let c2 = c.get();
    println!("{c1}");
    println!("{c2}");
}
