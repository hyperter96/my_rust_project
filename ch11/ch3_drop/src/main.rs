
struct Dog {
    name: String,
    // count: i32,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("{} leave", self.name);
        // self.count -= 1;
    }
}
fn main() {
    let a = Dog{name: String::from("wangcai")};
    {
        let b = Dog{name: String::from("dahuang")}; // 离开作用域时调用drop
    }
    drop(a); // 提前释放a
    println!("Hello, world!");
}
