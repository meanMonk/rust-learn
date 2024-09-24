use crate::greet;

mod tuples;
mod structs;

pub fn main() {
    greet::greet("Let's Learn Structs, Enums and Matching!");
    
    tuples::main();
    structs::main();
    
}