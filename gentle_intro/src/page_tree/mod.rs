use crate::greet;

mod file_reading;
mod file_path_directories;


pub fn main() {
    greet::greet("Let's Learn: Chapter 3 Filesystem and Process!");
    file_reading::main();
    file_path_directories::main();
}
