use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} in spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {} in main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 等待子线程结束，结束以后才会往下执行
    println!("hello world"); // hello world 在子线程结束以后才打印
}
