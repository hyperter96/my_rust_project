use std::collections::VecDeque;

// 多态
trait Driver {
    fn drive(&self);
}

struct Car;

impl Driver for Car {
    fn drive(&self) {
        println!("Car is driving")
    }
}
struct SUV;

impl Driver for SUV {
    fn drive(&self) {
        println!("SUV is driving")
    }
}

fn road(vehicle: &dyn Driver) {
    vehicle.drive()
}

// 继承思想
// 单向特质
trait Queue {
    fn len(&self) -> usize;
    fn push_back(&mut self, n: i32);
    fn pop_front(&mut self) -> Option<i32>;
}

// 双向特质
trait Deque: Queue {
    fn push_front(&mut self, n: i32);
    fn pop_back(&mut self) -> Option<i32>;
}

#[derive(Debug)]
struct List {
    data: VecDeque<i32>,
}

impl List {
    fn new() -> Self {
        let data = VecDeque::<i32>::new();
        Self{data}
    }
}

impl Deque for List {
    fn push_front(&mut self, n: i32) {
        self.data.push_front(n)
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.data.pop_back()
    }
}

impl Queue for List {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.data.pop_back()
    }

    fn push_back(&mut self, n: i32) {
        self.data.push_back(n)
    }
}

fn main() {
    road(&Car); // Car is driving
    road(&SUV); // SUV is driving

    let mut l = List::new();
    l.push_back(1);
    l.push_front(0);
    println!("{:?}", l); // List { data: [0, 1] }
    l.push_front(2);
    println!("{:?}", l); // List {data: [2, 0, 1] }
    l.push_back(3);
    println!("{:?}", l); // List {data: [2, 0, 1, 3]}
    println!("{}", l.pop_back().unwrap()); // 3
    println!("{:?}", l); // List {data: [2, 0, 1] }
}
