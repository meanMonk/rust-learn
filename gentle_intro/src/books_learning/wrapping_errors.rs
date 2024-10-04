use std::{error::{self, Error}, fmt::Display, num::ParseIntError};




#[derive(Debug)]
enum DoubleError {
    EmptyVec, 
    Parse(ParseIntError)
}


impl Display for DoubleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DoubleError::EmptyVec => write!(f, "please use vec with atleast one element"),
            DoubleError::Parse(..) => write!(f, "provided string could not be parsed into Int")
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None, 
            DoubleError::Parse(ref e) => Some(e)
        }
    }
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

type ResultDoubleError<T>= std::result::Result<T, DoubleError>;

fn tripple_first(vec: Vec<&str>) -> ResultDoubleError<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(3 * parsed)
}

fn print(result :ResultDoubleError<i32>)  {
    match result {
        Ok(s) => println!("tripple of first element {}",s),
        Err(e) =>  {
            println!("Error {}", e);
            if let Some(source) = e.source() {
                println!("Error caused by: {source}");
            }
        }
    }
}

pub fn main() {
    let numbers = vec!["23","24","25"];
    let empty = vec![];
    let strings = vec!["tofu", "23","43"];
    
    print(tripple_first(numbers));   
    print(tripple_first(empty));   
    print(tripple_first(strings));      
}