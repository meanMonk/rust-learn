/**
 * Primit types
 * Integers: u8, i8, u16,32,64,128 i16,32,64,128
 *  u -> only positive
 *  i -> positive + negative
 * Floats : f32 , f64 (default)
 * Boolean (true)
 * Characters (Char) -> declare with single qoute ''
 * Tuples
 * Arrays
 * 
 * 
 * // Rust is statically type lang, which means that is must know that types of all 
 * variables at compile time, however the compiler can usually infer what type we want to use based on the value 
 * and how we use it.
 * 
 * 
*/


pub fn run() {
  //Default is i32
  let alpha = 1;
  
  // default is f64
  let beta = 1.6;
  
  // add explicit type
  
  let gamma: i64 = 45454544545;
  
  // Find the max size 
  println!("max 32: {}", std::i32::MAX);
  println!("min 32: {}", std::i32::MIN);
  println!("max i64: {}", std::u64::MAX);
  println!("min i64: {}", std::u64::MIN);
  
  // boolean,
  let is_active = true;
  
  // boolean from expression
  let is_greater = 10 > 5;
  
  // char literals
  
  let a1 = 'a';
  // can be unicode for emoji faces.
  let face = '\u{1F603}';
  
  println!("{:?}", (alpha, beta, gamma,is_active, is_greater, a1, face));
  
}