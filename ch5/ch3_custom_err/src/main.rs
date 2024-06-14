use std::fmt::write;

#[derive(Debug)]
struct MyError {
    detail: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom Error: {}", self.detail)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.detail
    }
}

fn func_err() -> Result<(), MyError> {
    Err(MyError{
        detail: "Custom Error".to_owned(),
    })
}

fn func_ok() -> Result<(), MyError> {
    Ok(())
}

fn main() -> Result<(), MyError> {
    match func_ok() {
        Ok(_) => println!("func ok"),
        Err(err) => println!("Error: {}", err),
    }
    func_ok()?;
    println!("ok");
    match func_err() {
        Ok(_) => println!("func ok"),
        Err(err) => println!("Error: {}", err),
    }
    func_err()?;
    println!("oo");
    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     match func() {
//         Ok(_) => println!("func ok"),
//         Err(err) => println!("Error: {}", err),
//     }
//     func()?;
//     println!("oo");
//     Ok(())
// }