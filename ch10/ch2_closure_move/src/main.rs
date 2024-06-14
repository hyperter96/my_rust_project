fn main() {
    // Fn不可变引用获取外部参数
    let s1 = String::from("111111111");
    let s2 = String::from("222222222");

    let fn_func = |s| {
        println!("{s1}");
        println!("I am {s}");
    };
    
    fn_func("yz".to_owned()); // Fn不可变引用，所有权仍然保留
    fn_func("原子".to_owned());
    println!("{s1} {s2}"); // 111111111 222222222

    // FnMut可变引用获取外部参数，匿名函数中的外部参数存在修改
    let mut s1 = String::from("111111111");
    let mut s2 = String::from("222222222");

    let mut fn_func = |s| {
        s1.push_str("😊");
        s2.push_str("😊");
        println!("{s1}");
        println!("I am {s}");
    };
    
    fn_func("yz".to_owned()); // FnMut可变引用，所有权仍然保留
    fn_func("原子".to_owned());
    println!("{s1} {s2}"); // 111111111😊😊 222222222😊😊

    // 所有权转移
    let s1 = String::from("1111");
    let fn_Once_func = || {
        println!("{s1}");
        std::mem::drop(s1);
    };
    fn_Once_func(); // 1111
    // println!("{s1}");

    // 使用关键字move，捕获闭包外的环境变量所有权移至闭包内
    let s1 = String::from("1111");
    let move_fn = move || {
        println!("{s1}");
    };
    move_fn(); // 1111
    // println!("{s1}");


}
