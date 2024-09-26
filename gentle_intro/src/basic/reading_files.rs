// Reading From Files
// `expect` is like `unwrap` but gives custom error message.

// `File::open` returns `Option` that may contain something or nothing.

use std::{
    env,
    fs::File,
    io::{self, Read},
};

fn reading_one() {
    let file_name = env::args().nth(1).expect("please suppy file name!");

    let mut file = File::open(&file_name).expect("can't open the file!");

    let mut text = String::new();

    file.read_to_string(&mut text)
        .expect("can't read the file!");

    println!("file had {} bytes", text.len());
}

fn good_or_bad(good: bool) -> Result<i32, String> {
    if good {
        Ok(2)
    } else {
        Err("bad".to_string())
    }
}

fn read_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut text = String::new();

    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

// read to string with `?` operators.
// 2017 was good for Rust question mark became the stable
// old code it was with macro `try!()`

fn read_to_string_one(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn read_file_two() {
    let file_name = env::args().nth(1).expect("please supply filename");

    let text = read_to_string(&file_name).expect("bad file name!");

    println!("file had {} bytes", text.len());
    println!("file has content {:?}", text.to_string());

    let text1 = read_to_string_one(&file_name).expect("bad file name!");

    println!("file had {} bytes", text1.len());
    println!("file has content {:?}", text1.to_string());
}

pub fn main() {
    // reading_one();
    match good_or_bad(true) {
        Ok(n) => println!("Cool I got {n}"),
        Err(e) => println!("Huh, I just got {e}"),
    }

    read_file_two();
}
