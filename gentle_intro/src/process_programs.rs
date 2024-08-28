// Processes
// A fundamental need is for programs to run programs, or to launch process. 

// Your program spawn as many child process it likes, and as the name suggests they have specifal relationship with their parent.

use std::process::Command;

pub fn check_rustc() {
    let status = Command::new("rustc")
    .arg("-V")
    .status()
    .expect("no rustc?");

    println!("cool {} - code {}", status.success(), status.code().unwrap())
}

// as we are more interested in output and std errors which goes to terminal.
pub fn check_rustc_output() {
    
    let output = Command::new("rustc")
    .arg("-V")
    .output()
    .expect("no rustc?");

    if output.status.success() {
        println!("Ok!");
    }
    
    println!("len stdout {} stderr {}", output.stdout.len(), output.stderr.len());
}