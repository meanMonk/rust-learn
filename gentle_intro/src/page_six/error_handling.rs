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
    // we need to create our own custom error type. 
    basic requirement is:
    1/ may implement `Debug`
    2/ may implement `Display`
    3/ may implement `Error`

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
    raises_my_error(yes)?; // ? is fine as itself to from.
    let x: f64 = s.parse()?; // this will convert the parseFloatErrors to MyError
    Ok(x)
}

fn error_handling_three() {
    println!(" => {:?}",parse_f64("42", false));
    println!(" => {:?}",parse_f64("42", true));
    println!(" => {:?}",parse_f64("?42", false));
}

//  not too complicated although a little long-winded
// to write From conversions for all the other error types that need to play nice with MyError.
// there is always another way to peel the avacado.
// Error handlings is crucial when you package your droppings as cargo crate!

// NOTE: currently `?` operator only work for `Result` not for `Option`
// NOTE: Option has `ok_or_else` which converts it to `Result`
// 
// converting Option<String> to Result<String>.
// There are 2 `Option` methods `ok_or` and `ok_or_else`
// alternative to `ok_or` method. is use of `match`.
/* 
    let file = match args().skip(1).next() {
        Some(s) => s,
        None => bail!("error for provided file")
    }
*/ 

// error chaining can be does easily. 
// crate to use `error_chain`;
/* 
    // non-specific error
    let f = File::open(&file)?;

    // a specific chained error
    let f = File::open(&file).chain_err(|| "unable to read the damn file")?;

*/

pub fn main() {
    println!("Error handling!");
    error_handling_one();
    error_handling_two();
    error_handling_three();
}