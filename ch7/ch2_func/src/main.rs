// 交换
fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point{x, y}
    }

    // 方法, 加引用是为了防止当T是String的时候，返回以后所有权丢失
    fn get_coordinates(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}
fn main() {
    let result = swap(0, 1);
    println!("{:?}", result); // (1, 0)

    let str2 = swap("hh", "tt");
    println!("str2.0 {} str2.1 {}", str2.0, str2.1); // str2.0 tt str2.1 hh
    let str2 = swap(str2.0, str2.1);
    println!("str2.0 {} str2.1 {}", str2.0, str2.1); // str2.0 hh str2.1 tt

    let i32_point = Point::new(2, 3);
    let f64_point = Point::new(2.0, 3.0);
    let (x1, y1) = i32_point.get_coordinates();
    let (x2, y2) = f64_point.get_coordinates();
    println!("i32 point: x={} y={}", x1, y1);
    println!("f64 point: x={} y={}", x2, y2);

    // 最好用String, 不用&str
    let string_point = Point::new("x".to_owned(), "y".to_owned());
    println!("string point x={} y={}", string_point.x, string_point.y);

}
