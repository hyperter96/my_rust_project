use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });
    println!("1 leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 1 leaf strong: 1, weak: 0
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade()); // leaf parent: None

    let branch = Rc::new( Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![Rc::clone(&leaf)]), // branch作为父节点拥有leaf作为子节点
    });
    println!("1 branch strong: {}, weak: {}", Rc::strong_count(&branch), Rc::weak_count(&branch)); // 1 branch strong: 1, weak: 0
    println!("2 leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 2 leaf strong: 2, weak: 0

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // leaf节点的parent指向branch父节点
    println!("2 branch strong: {}, weak: {}", Rc::strong_count(&branch), Rc::weak_count(&branch)); // 2 branch strong: 1, weak: 1
    println!("leaf parent: {:#?}", leaf.parent.borrow().upgrade()); 
    // leaf parent: Some(
    //     Node {
    //         value: 5,
    //         parent: RefCell {
    //             value: (Weak),
    //         },
    //         child: RefCell {
    //             value: [
    //                 Node {
    //                     value: 3,
    //                     parent: RefCell {
    //                         value: (Weak),
    //                     },
    //                     child: RefCell {
    //                         value: [],
    //                     },
    //                 },
    //             ],
    //         },
    //     },
    // )
}
