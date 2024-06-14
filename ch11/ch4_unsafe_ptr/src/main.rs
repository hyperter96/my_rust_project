fn main() {
    // 指针不可变
    let x: usize = 1;
    let raw_ptr = &x as *const usize;

    // 指针可变
    let mut y: usize = 2;
    let raw_mut_ptr = &mut y as *mut usize;

    let some_usize = unsafe {
        *raw_ptr
    };
    println!("some usize: {some_usize}"); // some usize: 1

    let some_mut_usize = unsafe {
        *raw_mut_ptr
    };
    println!("some mut usize: {some_mut_usize}"); // some mut usize: 2
}
