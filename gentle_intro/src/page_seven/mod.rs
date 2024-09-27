// Thread, Networking and Sharing

use crate::greet;

mod borrow_checker;

pub fn main() {
    greet::greet("Thread, Networking and Sharing");
    
    borrow_checker::main();
}