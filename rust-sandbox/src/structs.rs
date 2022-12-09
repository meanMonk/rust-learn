/***
 *  // Structs
 *  ---  used to create the custom data types.
 */

 // Traditional Struct
 
//  struct  Color {
//   red: u8,
//   green: u8,
//   blue: u8
// }

// Tuple Struct.

// struct Color(u8,u8,u8);
 
struct  Person {
  f_name: String,
  l_name: String,
}


impl Person {
  /// Construct person
  /// 
  
  fn new(f: &str, l:&str) -> Person {
    return Person { f_name: f.to_string(), l_name: l.to_string() }
  }
  // get full name
  fn full_name(&self) -> String {
   return  format!("{} {}", self.f_name, self.l_name);
  }
  
  
  // set last name
  
  fn set_last_name(&mut self, last: &str) {
    self.l_name = last.to_string();
  }
  
  
  // Name to tuple.
  
  fn to_tuple(self) -> (String, String) {
    return (self.f_name, self.l_name)
  }
  
}

pub fn run() {
  
  /* let mut c = Color {
    red: 255, 
    green: 0, 
    blue: 0,   
  };
  
  c.blue = 255;
  
  println!("Color: {} {} {}", c.red, c.blue, c.green)
   */
  
   
   /* let mut c =  Color(255, 0, 0);
   
   c.2 = 255;
   
   println!("Color TUples: {} {} {}", c.0, c.1, c.2) 
   
   */
  let mut p = Person::new("John", "Doe") ;
  
  println!("Person name  {} {} ", p.f_name, p.l_name);
  println!("Person full name   {} ", p.full_name());
  
  p.set_last_name("Marry");
  println!("Person full name   {} ", p.full_name());
  
  println!("Tuple full name {:?} ", p.to_tuple());
   
  
 }