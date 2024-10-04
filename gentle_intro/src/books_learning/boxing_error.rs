

// boxing errors.

use std::error; 
use std::fmt;

use json::number;


type ResultOfError<T> = std::result::Result<T, Box<dyn error::Error>>;


#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}


fn double_first_number(vec: Vec<&str>) -> ResultOfError<i32> {
    vec
        .first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>().map_err(|e| e.into())
            .map(|i| 2 * i)
        })
}


fn print(result: ResultOfError<i32>) {
    match result {
        Ok(n) => println!("double of first element {n}"),
        Err(e) => println!("Error {}",e)
    }
}


pub fn main() {
    let numbers = vec!["23","24","25"];
    let empty = vec![];
    let strings = vec!["tofu", "23","43"];
    
    print(double_first_number(numbers));   
    print(double_first_number(empty));   
    print(double_first_number(strings));   
}