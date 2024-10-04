

// boxing errors.

use std::error; 
use std::fmt;


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
            s.parse::<i32>()
            .map_err(|e| e.into())
            .map(|i| 2 * i)
        })
}

fn double_first_with_que (vec: Vec<&str>) -> ResultOfError<i32> {
    
    let first  = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
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
    
    // print(double_first_number(numbers));   
    // print(double_first_number(empty));   
    // print(double_first_number(strings));   
    
    println!("with `?` operator!");
    
    print(double_first_with_que(numbers));   
    print(double_first_with_que(empty));   
    print(double_first_with_que(strings));   
}