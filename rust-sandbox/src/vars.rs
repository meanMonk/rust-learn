// variable is to store some primitive values or reference to data.
// rust is block-scope language.
// variables are immutable in rust.

pub fn run() {
  // declaring let 
  
  let name = "Rust";
  
  let mut age = 30;
  
  
  println!(" I started learning {} at the age of {}", name, age);
  age = 31;
  println!("I started coding in {} at the age of {}", name, age);
  
  
  // assign mulitiple vars once 
  let  (_name, _age) = ("Shiv", 1.2);
  
  
  println!("{} - {}", _name, _age)
}