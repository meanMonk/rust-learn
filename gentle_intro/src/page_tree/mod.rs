use crate::greet;

mod file_reading;


pub fn main() {
    greet::greet("Let's Learn: Chapter 3 Filesystem and Process!");
    file_reading::main();
}
