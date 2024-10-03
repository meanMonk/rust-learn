// # Error Hanlding.
// Rust has number of features for handling the situations in which something goes wrong.
// In many cases, Rust requires you to acknowledge the possibility of an error take action before your code compile.

// Rust groups errors majorly in 2 categories.
// 1. recoverable
//    - such as file not found error, we most likely want to report the problem to user and retry the operation
// 2. unrecoverable
//    - are always symptoms of bugs, such as trying to access a location beyond the end of an array. 
// so we want to stop the program immediately.
// Rust does't have exceptions, 
// - for recoverable : has type Result<T, E>
// - for unrecoverable : `panic!` macro stops execution when the program encounters unrecoverable error.


// #Unrecoverable Errors
// there are two ways to cause panic in practice.
// 1/ accessing the array element pass the end.
// 2/ invoking `panic!` macro
//  - By default, these panics will print a failure message, unwind, clean up the stack, and quit.


// Recoverable Errors.
// most errors aren't serious enough to require the program to stop entirely.
// sometime functions fails it's for a reason that you can easily interpret and respond to.
// 
/* 
    // with match
    
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

*/

/* 
    

// instead of using match
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

*/

// shortcut for panic on error: unwrap or expect.
// `unwrap` is shortcut like match. > in case result invoke Ok(res) in case of error `unwrap` call panic! macro for us.
// 
/* 
    use std::fs:File;
    fn main() {
        let greeting_file =  File::open("hello.txt").unwrap();
    }
*/

// `expect`
// the `expect` method lets us also choose the panic! error message.
// using `expect` instead of `unwrap` and providing good error message can convey your intent make tracking down source panic easier
/* 
    //panic with expec.
    fn main( ) {
        let greeting_file = File::open("hello.txt").expect("unable to open file!");
    }
    
*/

/* 

    In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed. 
    That way, if your assumptions are ever proven wrong, you have more information to use in debugging.
*/

/* 
    #Propogating Errors.
     - When function's implementation calls something that might fail, instead of handling the error within the function
     itself you can return the error to the calling code so that it can decide what to do.

*/

use std::{fs::{self, File}, io::{self, Read}};

/* 
    // function can be written in short way.
    // returns Result<String, io::Error>
        //  if success return Ok(username)
        //  if failed return Err(io::Error)
        
    // it's upto calling code to decide what to do with those error values. 
    // this way we propogate all the success / error info upward for it to handle appropriately.
*/

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e) // we use `return` to return early out of function instead of panic.
    };
    
    let mut username = String::new();
    // `read_to_string` might fails so we need another match to handle that `Result`
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e) // returning error wrapped in Err() but we don't need `return` keyword as it's last line of function.
    }
    
}

// Prepare linkedin twitter post for `?` operators with use cases.
// A shortcut for propagating errors: the `?` operators
// using operator.
// `?` operator call the `from` function
// in short ? will take care of conversion of error by calling `from` on traits.

fn read_username_from_file_with_que() -> Result<String, io::Error>{
    let mut username_file = File::open("username.txt")?; // `?` work same as above match
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// using `?` operator we can shorten the code as well.

fn read_username_from_file_short() -> Result<String, io::Error>{
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    
    Ok(username)
}

// we can even make shorter using the `fs::read_to_string` fn.

fn read_username_from_file_shorten() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

// where the `?` Operator can be used in functions whose return type is compatible with the value `?` is used on.
// This is because the `?` operator is defined to perform an early return of a value out of the function.


// `?` operator can also be used Option<T>
// `?` Option<T> if success return Some(T) and if failes None.

// takes arg as string slice argument calls the lines method on it.
// lines() return iterators so invoking next() to get line
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

/* 
NOTE:

 - 1/ you can use the ? operator on a Result in a function that returns Result, 
 - 2/ you can use the ? operator on an Option in a function that returns Option
 but can't mix and match `?` operator won't convert `Result` to `Option` automatically.
 
 to convert we can use below method to do the conversion explicitly.
  - 1/ `ok` method on Result.
  - 2/ `ok_or` method on Option
 
*/
/* 
    The `main` is special function because it's the entry point and exit point of an executable program so there are
    restrictions on what it's return type can be for the program to behave as expected.
    
    Luckly main can also return `Result<(), E>` so we can update or shorter function like
    
    fn main() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("file.txt")?;
        Ok(());
    }
    
    NOTE: The `Box<dyn Error>` type is a trait object, that u can read any kind of Error.
    
*/



// caling panic!
pub fn main() {
    // panic!("crash and burn");
    // use of panic with code.
    
    // let v = vec![1,2,4,5];
    
    // v[99];
}