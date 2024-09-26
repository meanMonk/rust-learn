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

use std::io::prelude::*;




pub fn main() {
    
}