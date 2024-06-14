use std::ops::Add;
use std::ops::Sub;
// 编译时
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// T的这样类型，它可以执行相加的操作
impl<T> Add for Point <T>
    where T: Add<Output = T> 
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
    
}

// 相减
impl <T> Sub for Point <T>
    where T: Sub<Output = T>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
fn main() {
    let i1 = Point{x: 1, y: 2};
    let i2 = Point{x: 1, y: 3};
    let sum = i1 + i2;
    println!("{:?}", sum); // Point { x: 2, y: 5 }

    let i1 = Point{x: 1, y: 2};
    let i2 = Point{x: 1, y: 3};
    let sub = i1 - i2;
    println!("{:?}", sub); // Point { x: 0, y: -1 }
}

