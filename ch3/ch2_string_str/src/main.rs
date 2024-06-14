#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    color: String,
}

fn print(data: &str) {
    println!("{}", data);
}

fn print_string_borrow(data: &String) {
    println!("{}", data);
}

fn main() {
    let name = String::from("Value C++");
    let course = "Rust".to_owned();
    let new_name = name.replace("C++", "CPP");
    take_ownership(new_name.clone());
    println!("{name} {course} {new_name}");
    let rust = "\x52\x75\x73\x74"; //ascii码
    println!("{rust}");
    let color = "green".to_string();
    // let people = Person {
    //     name: name,
    //     color: color,
    //     age: 8,
    // };
    // take_ownership_struct(people);
    print("red");
    print(&name);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn take_ownership_struct(s: Person) {
    println!("{:?}", s);
}


