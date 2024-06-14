
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn change_i32(mut x: i32) {
    x = x + 4;
    println!("fn {x}");
}

fn modify_i32(x: &mut i32) {
    *x += 4;
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn print_point(point: Point) {
    println!("point x: {}", point.x);
}

fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("c: {c}");
    let mut x = 1; // 实参
    change_i32(x); // fn 5
    println!("change x: {x}"); // change x: 1
    // 实参x和change_i32函数中的形参改变没有关系
    modify_i32(&mut x); // 可变借用
    println!("modify x: {x}"); // modify x: 5
    let s = Point{x: 1, y: 2};
    print_point(s); // 所有权消失
    println!("{}", s.x); // 实现Copy
}
