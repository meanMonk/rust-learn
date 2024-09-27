use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

use crate::greet;

// Process:
// A fundamental need is for the programs to run programs.
// to lancuh process.
// you program can spawn as many child process it likes.
// as name says they special relationship with their parent.

// to run the program is straightforward using the `Command` struct which builds args to pass to program.

fn process_one() {
    let status = Command::new("rustc").arg("-V").status().expect("no rustc!");

    println!("cool {} code {}", status.success(), status.code().unwrap());

    /*
    There are three possibilities:
        - program didn't exist, was bad, or we were not allowed to run it
        - program ran, but was not successful - non-zero exit code
        - program ran, with zero exit code. Success!

     */
}

// we are very intersted in capturing the output so there is method `output`
fn process_two() {
    let output = Command::new("go").arg("version").output().expect("no go?");

    if output.status.success() {
        println!("Ok!");
    }

    println!(
        "len stdout {:?} stderr {:?}",
        output.stdout.len(),
        output.stderr.len()
    );
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

// Function which runs the program using the shell
fn shell(cmd: &str) -> (String, bool) {
    let cmd = format!("{} 2>&1", cmd);
    println!("cmd => {cmd}");
    let shell = if cfg!(windows) { "cmd.exe" } else { "/bin/sh" };
    let flag = if cfg!(windows) { "/c" } else { "-c" };
    let output = Command::new(shell)
        .arg(flag)
        .arg(&cmd)
        .output()
        .expect("no shell?");

    (
        String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string(),
        output.status.success(),
    )
}

fn shell_success(cmd: &str) -> Option<String> {
    let (output, success) = shell(cmd);
    if success {
        Some(output)
    } else {
        None
    }
}

fn process_three() {
    let res = shell_success("which go");
    println!("{:?}", res)
}

// upto now program simply wait for child process to finish.
// if use the spawn method then we return immediately, and must explicityly wait for it to finish or go off and do something else in meantime!
// example show how to suppress both standard out and standard error.

fn process_four() {
    let child = Command::new("rustc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("no rustc ?");

    // can kill process with
    // let _ = child.kill()
    let res = child.wait_with_output();

    println!("res {:?}", res);
}

pub fn main() {
    greet::greet("Learn process handling RUST!");

    process_one();

    process_two();

    process_three();

    process_four();
}
