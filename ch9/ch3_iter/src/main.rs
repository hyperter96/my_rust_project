fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    // iter() 不可变引用的迭代器
    for &item in vec.iter() {
        println!("{}", item);
    }
    println!("{:?}", vec); // [1, 2, 3, 4, 5]

    // 可变引用
    let mut vec = vec![1, 2, 3, 4, 5];
    for item in vec.iter_mut() {
        *item *= 2;
    }
    println!("{:?}", vec); // [2, 4, 6, 8, 10]

    // 所有权转移
    let vec = vec![1, 2, 3, 4, 5];
    for item in vec.into_iter() {
        println!("{}", item);
    }
    // println!("{:?}", vec);
}
