// Error handling
// If you can't use the question-mark operator error handling can be clumsy.
// To achieve happiness we need to return a `Result` which can accept any error.
// All errors implement the trait `std::error:Error` so any error can convert to `Box<Error>`


// Currently, the question-mark operator `?` only works for Result, not Option, 
// this is a feature, not a limitation. 
// Option has a `ok_or_else` which converts itself into a Result. 
// For example, say we had a HashMap and must fail if a key isn't defined:
/* 
        let mut map = HashMap::new();
        map.insert("two",2);
        let val = map.get("my_key")
        .ok_or_else(|| MyError::new("my_key not defined"))?;

        
        `.ok_or_else`: accepts the closure
*/

// let's say we needed to handle both the i/o errors and errors from converting strings into numbners.

use core::fmt;
// box-error.rs
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::num::ParseFloatError;

fn run(file: &str) -> Result<String,Box<dyn Error>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn error_handling_one() {
    let res: Result<String, Box<dyn Error>> = run("test.txt");
    println!("file reading {:?}", res.unwrap());
}


// it's easy to create the shortcut for result type.
type BoxResult<T> = Result<T, Box<dyn Error>>;

/*
    // our programs will have application specific error conditions,
    // we need to create our own error type. requirement
    
    1/ may implement `Debug`
    1/ may implement `Display`
    1/ may implement `Error`

*/ 

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{ details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl  Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}


fn raises_my_error(yes: bool) -> Result<(), MyError> {
    if yes {
        Err(MyError::new("broken by boolean"))
    } else {
        Ok(())
    }
}

fn error_handling_two() {
   let res = raises_my_error(true);
   println!("{:?}",res.unwrap_err()); // to unwrap error
}

// Other Errors : 
// `ParseFloatError` implements `Error` so `description()` is defined.

impl From<ParseFloatError> for MyError{
    fn from(value: ParseFloatError) -> Self {
        MyError::new(&value.to_string())
    }
}

fn parse_f64(s: &str, yes:bool) -> Result<f64, MyError> {
    raises_my_error(yes)?;
    let x: f64 = s.parse()?;
    Ok(x)
}

fn error_handling_three() {
    println!(" => {:?}",parse_f64("42", false));
    println!(" => {:?}",parse_f64("42", true));
    println!(" => {:?}",parse_f64("?42", false));
}

pub fn main() {
    println!("Error handling!");
    error_handling_one();
    error_handling_two();
    error_handling_three();
}