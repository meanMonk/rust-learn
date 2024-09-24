use crate::greet;

mod tuples;
mod structs;
mod traits;
mod generic_functions;
mod enums;

pub fn main() {
    greet::greet("Let's Learn Structs, Enums and Matching!");
    
    tuples::main();
    structs::main();
    traits::main();   
    generic_functions::main();
    enums::main()
}