use std::io::{stdout, BufWriter};

use ferris_says::say;



pub fn greet(message: &str) {
    let width = message.chars().count();
  
  let writer = BufWriter::new(stdout());
  
  say(message, width, writer).unwrap();
}
