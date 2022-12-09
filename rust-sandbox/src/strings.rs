// primitive str = immutatble fixed length string somewhere in memory
// string = growable, heap allocated data structure = use when you need to modify or own 
// string data

/**
 * More details about the available methods on string
 * 
 * checkout the documents
 * https://doc.rust-lang.org/std/string/struct.String.html
 * 
 * 
*/


pub fn run() {
  let mut hello = String::from("Hello ");
  
  println!("{}", hello);
  // get length
  println!("Length - {}", hello.len());
  // push & push_str
  // push is for pushing unicode chars.
  hello.push('W');
  
  // push_str is for pushing the string
  hello.push_str("orld!");
  
  // capacity in bytes
  
  println!("Capacity : {}", hello.capacity());
  
  // check isEmpty
  println!("IsEmpty : {}", hello.is_empty());
  
  // check does contains
  println!("does contain : {}", hello.contains("llo"));
  
  // replace in string.
  println!("replace : {}", hello.replace("World", "Demo"));
 
  
  // loop through the string by whitespace.
  for word in hello.split_whitespace() {
    println!("{}",word);
  }
  
  // create string with capacity
  
  let mut s = String::with_capacity(10);
  
  s.push('a');
  s.push('b');

  // Assserting testing.
  
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
  
  println!("{}", s);
}