use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用

    let z = Box::new(x);
    assert_eq!(5, *z);

    // 需要实现Deref特质
    let a = MyBox::new(x);
    assert_eq!(5, *a);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 解引用的强制多态：将MyBox变为&String, 再将String解引用，变为&str, 
    println!("hello world");
}

fn hello(s: &str) {
    println!("hello {s}");
}


