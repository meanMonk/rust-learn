// Thread, Networking and Sharing

use crate::greet;

mod borrow_checker;
mod shared_refs;
mod multithreading;
mod channels;
mod higher_level;

/* 
    To Learn:
    
     - Synchronization
     - Shared references threading equivalent to RefCell is Mutex.
     - Higher Level Operation

*/

pub fn main() {
    greet::greet("Thread, Networking and Sharing");
    
    borrow_checker::main();
    shared_refs::main();
    multithreading::main();
    channels::main();
    // higher_level::main();
}