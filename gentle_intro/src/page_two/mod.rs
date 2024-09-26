use crate::greet;

mod closures;
mod enums;
mod generic_functions;
mod struct_dynamic_data;
mod structs;
mod traits;
mod tuples;

pub fn main() {
    greet::greet("Let's Learn Structs, Enums and Matching!");

    tuples::main();
    structs::main();
    traits::main();
    generic_functions::main();
    enums::main();
    closures::main();
    struct_dynamic_data::main();
}
