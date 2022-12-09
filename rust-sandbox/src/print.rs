
/**
 * PRINT: 
 * 
 *  Printing is handled by a series of macros defined in std::fmt some of which include:
 *  - format!: write formatted text to String
 *  - print!: same as format! but the text is printed to the console (io::stdout).
 *  - println!: same as print! but a newline is appended.
 *  - eprint!: same as print! but the text is printed to the standard error (io::stderr).
 *  - eprintln!: same as eprint! but a newline is appended.
 * 
 * 
*/


pub fn run() {
  /* ****** Learning ****** */
  // In general, the `{}` will be automatically replaced with any
  // arguments. These will be stringified.
  
  
  
  // Print to console
  println!("Hello from print.rs file!");
  
  // basic formatting
  println!("Number: {}", 1);
  
  println!("{} learning {} ", "Sahil","Rust Lang!");
  
  // Positional arguments can be used. Specifying an integer inside `{}`
  // determines which additional argument will be replaced. Arguments start
  // at 0 immediately after the format string
  println!("{0} learning {1} and {0} is going to play Programming", "sahil", "rust");
  
  // As can named arguments.

  println!("{name} like to learn {tech}",
    name = "Sahil",
    tech = "Rust"
  );
  
  // placeholder traits
  // Different formatting can be invoked by specifying the format character after a
  // `:`. 
  // {:b}  {:x}  {:o} {:x} {:X}
  println!("Binary {:b} Hex {:x} Octal {:o}", 10,10,10);
  
  // placeholder for debug traits
  
  println!("{:?}", (12, true, "hello"));
  
  // math operation
  println!("10+10 = {}", 10+10)
  
  
}