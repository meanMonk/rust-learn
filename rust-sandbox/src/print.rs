pub fn run() {
  // Print to console
  println!("Hello from print.rs file!");
  
  // basic formatting
  println!("Number: {}", 1);
  
  println!("{} learning {} ", "Sahil","Rust Lang!");
  
  // positional arguments 
  println!("{0} learning {1} and {0} is going to play Programming", "sahil", "rust");
  
  // Named arguments
  println!("{name} like to learn {tech}",
    name = "Sahil",
    tech = "Rust"
  );
  
  // placeholder traits
  println!("Binary {:b} Hex {:x} Octal {:o}", 10,10,10);
  
  // placeholder for debug traits
  
  println!("{:?}", (12, true, "hello"));
  
  // math operation
  println!("10+10 = {}", 10+10)
  
  
}