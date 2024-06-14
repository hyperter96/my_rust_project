use std::{sync::{Arc, Weak}, thread};

#[derive(Debug)]
struct Owner {
    name: String,
    dogs: Vec<Weak<Dog>>,
}

#[derive(Debug)]
struct Dog {
    owner: Arc<Owner>,
}

fn main() {
    let someone = Arc::new(Owner {
        name: "tom".to_string(),
        dogs: vec![],
    });

    for i in 0..10 {
        let someone = Arc::clone(&someone);
        let join_handle = thread::spawn(move || {
            let yellow_dog = Arc::new(Dog {
                owner: Arc::clone(&someone),
            });
            let black_dog = Arc::new(Dog {
                owner: Arc::clone(&someone),
            });
            println!("yellow dog owner: {}", yellow_dog.owner.name);
            println!("black dog owner: {}", black_dog.owner.name);
            println!("thread {i} end");
        });
        _ = join_handle.join();
    }
}
