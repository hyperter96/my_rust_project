fn main() {
    // vec
    let v = vec![1, 2, 3, 4, 5]; // intoIterator特质into_iter
    // 转换为迭代器
    let iter = v.into_iter(); // move 所有权转移 Iter，类似Iter Iterator的特质对象
    let sum: i32 = iter.sum();
    println!("sum：{}", sum); // sum：15
    
    // array
    let array = [1, 2, 3, 4, 5];
    let arr_iter = array.iter();
    let sum: i32 = arr_iter.sum();
    println!("sum：{}", sum); // sum：15
    println!("{:?}", array); // [1, 2, 3, 4, 5] 所有权没有转移
    // char
    let text = "hello world!";
    let text_iter = text.chars();
    let uppercase: String = text_iter.map(|c|c.to_ascii_uppercase()).collect();
    println!("uppercase: {}", uppercase); // uppercase: HELLO WORLD!
}
