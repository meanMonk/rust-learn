use crate::greet;

mod file_path_directories;
mod file_reading;
mod learn_json;
mod learn_process;
mod learn_serde_json;
mod modules;

pub fn main() {
    greet::greet("Let's Learn: Chapter 3 Filesystem and Process!");
    file_reading::main();
    file_path_directories::main();
    learn_process::main();
    modules::main();
    learn_json::main();
    learn_serde_json::main();
}
