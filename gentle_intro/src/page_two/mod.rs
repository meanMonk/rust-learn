use crate::greet;

mod tuples;
mod structs;
mod traits;

pub fn main() {
    greet::greet("Let's Learn Structs, Enums and Matching!");
    
    tuples::main();
    structs::main();
    traits::main();   
}