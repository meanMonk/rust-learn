// Another look at Reading files.

// better way to read the file line-by-line
// `fs::File` implements `io::Read` whihc is trait for anything readable.
// this traits provide below methods. with raw read no buffering.
//      - `read` : to fill a slice of u8
//      - `read_to_end` : to fill a vector of bytes with contents from the readable
//      - `read_to_string` : to fill a string : must be UTF-8 encoded.

// For buffer reading there is the `io::BufRead` trait with methods.
//  - `read_line` : 
//  - `lines` : iterator.
//  `io::BufReader` will provide an implementation of `io::BufRead` for any readable.

// easiest way to make all traits are visible.

use std::{fs::File, io::{self, BufRead, BufReader, Result, Write}};


fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;
    
    let reader =  io::BufReader::new(file);
    
    // `lines` being iterator so it's straightforward to read file into the vector of string 
    // using `collect` or `print out the lines with line number using enumerate iterator.
    for line in reader.lines().enumerate() {
        let string = line.1?;
        println!("-> {} = {:?} {:?}", line.0, string, string.len());
    }
    
    Ok(())
}

// efficient way of read_line 

fn read_all_lines_two(filename: &str) -> io::Result<()>{
    let file = File::open(&filename)?;
    let mut reader = BufReader::new(file);
    let mut buf =  String::new();
    
    while reader.read_line(&mut buf)? > 0 {
        {
            let line = buf.trim_end();
            println!("'{}'", line);
        }
        buf.clear();
    }
    
    Ok(())
    
}


fn read_and_write_to_other(sourcename: &str, targetname: &str) -> Result<()>{
    
    let sourcefile = File::open(&sourcename)?;
    let targetfile = File::create(&targetname)?;
    
    let mut reader = io::BufReader::new(sourcefile);
    
    let mut writer = io::BufWriter::new(targetfile);
    
    let mut buf = String::new();
    
    while reader.read_line(&mut buf)? > 0 {
        {
            write!(writer," 🚦 {}", buf)?;
        }
        buf.clear();
    }
    
    
    Ok(())
}

fn reading_one() {
    let res =  read_all_lines("sample.txt").expect("bad file name!");
    println!("File read by line {:?}", res);
    
    let res =  read_all_lines_two("sample.txt").expect("bad file name!");
    println!("Efficient way {:?}", res);
    
    let res = read_and_write_to_other("sample.txt", "test.txt").expect("something went wrong");
    println!("Data has been copied {:?}", res);
}

pub fn main() {
    reading_one();   
}