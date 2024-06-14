struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let box_point = Box::new(Point{x: 10, y: 20});
    println!("x:{}, y:{}", box_point.x, box_point.y);
}
