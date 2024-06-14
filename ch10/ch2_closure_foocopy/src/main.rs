#[derive(Debug, Copy, Clone)]
struct FooCopy {
    value: i32,
}

impl FooCopy {
    fn new(value: i32) -> Self {
        Self { value }
    }
    
    fn get(&self) -> i32 {
        self.value
    }
    
    fn increase(&mut self) {
        self.value += 1;
    }
}

fn is_FnMut<F: FnMut()>(c: &F) {}

fn is_Copy<F: Copy>(c: &F) {}
fn main() {
    let mut foo_copy = FooCopy::new(0);
  
    let mut c_with_move = move || {
        for _ in 0..5 {
            foo_copy.increase();
        }
        
        println!("foo_copy in closure(with move): {}", foo_copy.get());
    };
    
    c_with_move(); // foo_copy in closure(with move): 5
    println!("foo_copy out of closure(with move): {}\n", foo_copy.get()); // foo_copy out of closure(with move): 0

    let mut c_without_move = || {
        for _ in 0..5 {
            foo_copy.increase();
        }
        
        println!("foo_copy in closure(without move): {}", foo_copy.get());
    };
    
    is_FnMut(&c_with_move);
    is_Copy(&c_with_move);
    
    is_FnMut(&c_without_move);
    //is_Copy(&c_without_move); // Error
    
    c_without_move(); // foo_copy in closure(without move): 5
    println!("foo_copy out of closure(without move): {}\n", foo_copy.get()); // foo_copy out of closure(without move): 5
}
