//  Reading the Documentation
// Note: Special Relation ship between Vec<T> and &[T]
// Any method that works on slices will also work on vectors without explicit use of `as_slice` method.

/* 
    So I'd suggest you become familiar with all the 
        - iterator methods, 
        - because they are crucial to writing good Rust code without having to write loops
        - out all the time. 
        - As always, write little programs to explore iterator methods, 
        - rather than wrestling with them in the context of a more complicated program.
*/



use crate::greet;

mod learn_vectors;
mod learn_maps;
mod learn_sets;

pub fn main() {
    greet::greet("Stand Lib Functions!");
    
    learn_vectors::main();
    
    learn_maps::main();
    
    learn_sets::main();
}