struct MyString<'a> {
    text: &'a str, // 最好写String
}

impl<'a> MyString<'a> {
    fn get_length(&self) -> usize {
        self.text.len()
    }

    fn modify_data(&mut self) {
        self.text = "world";
    }
}

struct StringHolder {
    data: String,
}

impl StringHolder {
    fn get_length(&self) -> usize {
        self.data.len()
    }

    fn get_ref<'a>( &'a self) -> &'a String {
        &self.data
    }
}

fn main() {
    let str1 = String::from("value");
    let mut x = MyString{
        text: str1.as_str(),
    };
    println!("x before mod: {}", x.text); // x before mod: value
    x.modify_data();
    println!("x after mod: {}", x.text); // x after mod: world

    let holder = StringHolder{
        data: String::from("Hello"),
    };
    println!("{}", holder.get_ref()); // Hello

}
