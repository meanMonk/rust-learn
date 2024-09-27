// simple-error for Simple Errors
// The `simple-error` crates provides you with basic error types based on strings.
// few convenient macros it works with any Box<Error>

extern crate simple_error;

use std::error::Error;

use simple_error::bail;

type BoxResult<T> = Result<T, Box<dyn Error>>;

fn run(s: &str) -> BoxResult<i32> {
    
    if s.len() == 0 {
        bail!("empty string!")
    }
     
    Ok(s.trim().parse()?)
}


pub fn main() {
    println!("{:?}", run(""));
    println!("{:?}", run("23"));
    println!("{:?}", run("?23"));
}