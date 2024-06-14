struct Counter {
    number: i32,
}

impl Counter {
    fn new(number: i32) -> Self {
        Self {number}
    }

    fn get_number(&self) -> i32 {
        self.number
    }

    fn add(&mut self, increment: i32) {
        self.number += increment;
    }

    fn combine(c1: Self, c2: Self) -> Self {
        Self {
            number: c1.number + c2.number
        }
    }


}
fn main() {
    let mut c1 = Counter::new(11);
    println!("c1 number {}", c1.get_number());
    c1.add(2);
    println!("c1 number {}", c1.get_number());

    let c1 = Counter::new(15);
    let c2 = Counter::new(18);
    let c3 = Counter::combine(c1, c2);
    println!("c3 number {}", c3.get_number());

}
