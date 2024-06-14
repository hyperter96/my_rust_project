use std::num::ParseIntError;

fn find_first_even(numbers: Vec<i32>) -> Option<i32> {
    let first_even = numbers.iter().find(|&num| num % 2 == 0)?; // 使用?运算符前提条件是需要返回Option类型
    Some(*first_even)
}

// 传递错误
fn parse_numbers(input: &str) -> Result<i32, ParseIntError> {
    let val = input.parse::<i32>()?; // 使用?运算符如果发现error将提前返回
    Ok(val)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result_ok: Result<i32, &str> = Ok(32);
    let value = result_ok.unwrap();
    println!("{}", value); // 32

    let result_ok: Result<i32, &str> = Ok(32);
    let value = result_ok?;
    println!("{}", value); // 32

    let numbers = vec![1, 2, 3, 4, 5];
    match find_first_even(numbers) {
        Some(number) => println!("first even {}", number), // first even 2
        None => println!("no such number"),
    }

    match parse_numbers("d") {
        Ok(i) => println!("parsed {}", i),
        Err(err) => println!("failed to parse: {}", err), // failed to parse: invalid digit found in string
    }

    Ok(())
}
