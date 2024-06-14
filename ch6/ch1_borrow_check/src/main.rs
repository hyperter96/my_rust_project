fn main() {
    let mut s = String::from("Hello");
    // 不可变引用，可以有多个不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2); // Hello Hello

    // 可变引用只能有一个
    let r3 = &mut s;
    println!("{}", r3); // Hello

    let result: &str;
    {
        // result = "ff";
        let r4 = &s; // 给r4定义，r4的生命周期结束
        result = ff(r4); // 相当于给result初始化，result的生命周期还没结束
    }

    // println!("r4 {}", r4); // r4的所有权已经转移给result
    println!("{}", result); // Hello
}

fn ff<'a>(s: &'a str) -> &'a str {
    s
}
