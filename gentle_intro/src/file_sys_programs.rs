// use fs::File; which implements io::Read defines read methods.
// for buffered reading there is `io::BufRead` which gives `read_line` and return `lines` iterator
// fs::File also implements io::Write

use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{env, io};

struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String,
}

impl<R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines {
            reader: io::BufReader::new(r),
            buf: String::new(),
        }
    }

    fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>> {
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(nbytes) => {
                if nbytes == 0 {
                    None
                } else {
                    let line = self.buf.trim_end();
                    Some(Ok(line))
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn read_all_lines(filename: &str) -> io::Result<()> {
    println!("🤩 {:^30}", "READ_ALL_LINES");
    let file = File::open(&filename)?;

    let mut lines = Lines::new(file);
    
    // while loop doesn't provide index so we need to manage it by increamenting it.
    
    let mut index =0;
    while let Some(line) = lines.next() {
        let line = line?;
        println!("👀 {}", line);
        
        if index == 5 {
            break;
        }
        
        index += 1;
    }

    Ok(())
}

pub fn write_out(filename: &str) -> io::Result<()> {
    println!("🤩 {:^30}", "WRITE_OUT");
    let mut out = File::create(filename)?;
    write!(out, "answer is {}\n", 42)?;
    Ok(())
}

pub fn print_cargo_path() -> io::Result<()> {
    println!("🤩 {:^30}", "PRINT_CARGO_PATH");
    let home = env::home_dir().expect("home dir");

    let mut path = PathBuf::new();

    path.push(home);
    path.push(".cargo");

    println!("home path => {}", path.display());

    Ok(())
}

pub fn travel_to_home_dir() -> io::Result<()> {
    println!("🤩 {:^30}", "TRAVEL_TO_HOME_DIR");
    // read current dir.
    let mut current_path = env::current_dir().expect("can't access current dir");

    // looping on path as infinite.
    loop {
        println!("🛣️ -> {}", current_path.display());
        // break loop if pop returns nothing.
        if !current_path.pop() {
            break;
        }
    }

    Ok(())
}

// write program to search for the readme file from current project and travers back if not found.
// this is how `git` works
pub fn print_readme_path() -> io::Result<()> {
    println!("🤩 {:^30}", "PRINT_README_PATH"); 
    
    let mut current_path = env::current_dir().expect("Can't access current dir!");

    loop {
        current_path.push("README.md");
        if current_path.is_file() {
            println!("README path -> {}", current_path.display());
            break;
        } else {
            current_path.pop();
        }
        if !current_path.pop() {
            break;
        }
    }

    Ok(())
}


pub fn file_meta_data() {
    println!("🤩 {:^30}", "FILE_META_DATA");
    
    let mut path = env::current_dir().expect("can't access current dir");
    
    path.push("src");
    path.push("file_sys_programs.rs");
    
    if path.is_file() {
        match path.metadata() {
            Ok(data)=> {
                println!("Type {:?}", data.file_type());
                println!("Len {:?}", data.len());
                println!("per {:?}", data.permissions());
                println!("platform spec permission 🔒 {:o}", data.permissions().mode());
                println!("modified {:?}", data.modified());
            }, 
            Err(err) => println!("error {:?}", err)
        }
    } else {
        println!("Not file {}", path.display());
    }
    
}