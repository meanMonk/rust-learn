// Thread, Networking and Sharing

use crate::greet;

mod borrow_checker;
mod shared_refs;
mod multithreading;
mod channels;

pub fn main() {
    greet::greet("Thread, Networking and Sharing");
    
    borrow_checker::main();
    shared_refs::main();
    multithreading::main();
    channels::main();
}