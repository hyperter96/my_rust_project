// 字面量的生命周期都是一样的时候，性能相对慢一些
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// 返回的字面量的生命周期out是a和b的交集
fn longest_str<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'out str 
where
    'a: 'out,
    'b: 'out,
{
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


fn no_need(s: &'static str, s1: &str) -> &'static str {
    s
}

fn main() {
    println!("longest s: {}", longest("aabb", "ab")); // longest s: aabb
    let result: &str;
    {
        let r2 = "cba";
        result = longest_str("aabbcc", r2);
        println!("longest s of out: {}", result); // longest s of out: aabbcc
    }
    println!("no need {}", no_need("hh", "")); // no need hh
}
