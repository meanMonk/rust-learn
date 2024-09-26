use std::{env, os::unix::fs::MetadataExt, path::PathBuf};

use crate::greet;

// reading .cargof ile.

fn file_path_one() {
    let home = env::home_dir().expect("no home!");
    let mut path = PathBuf::new();

    path.push(home);
    path.push(".cargo");

    println!("cargo available at {}", path.display());
}

// print tree from current dir.

fn file_path_two() {
    let mut path = env::current_dir().expect("can't access current dir!");
    loop {
        println!(" {} ", path.display());
        if !path.pop() {
            break;
        }
    }
}

// search for Readme.md file.

fn file_path_three() {
    let mut path = env::current_dir().expect("can't access current dir!");

    loop {
        path.push("README.md");

        if path.is_file() {
            println!("GOT README {}", path.display());

            match path.metadata() {
                Ok(data) => {
                    println!("Type {:?}", data.file_type());
                    println!("Size {:?}", data.len());
                    println!("Permission {:?}", data.permissions());
                    println!("Modfied {:?}", data.modified());
                    println!("Created {:?}", data.created());
                    println!("Rights {:?}", data.mode());
                }
                Err(e) => println!("error {:?}", e),
            }

            break;
        } else {
            path.pop();
        }

        if !path.pop() {
            break;
        }
    }
}

pub fn main() {
    greet::greet("File Paths Directories!");

    file_path_one();
    file_path_two();
    file_path_three();
}
