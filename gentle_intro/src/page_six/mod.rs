use crate::greet;

mod error_handling;
mod simple_error;
pub fn main() {
    greet::greet("Learn  | Error Handling");
    
    error_handling::main();
    simple_error::main();
}