

use ferris_says::say;
use std::io::{stdout, BufWriter};


/* 
 this is type of block comments.
/*  */

*/

pub fn welcom() {
    
  let message = "Hello ! welcome to the ✋✋ RUST ✋✋";
  
  let width = message.chars().count();
  
  let writer = BufWriter::new(stdout());
  
  say(message, width, writer).unwrap();
}