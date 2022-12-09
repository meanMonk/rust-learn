/***
 *  // functions
 *  --- used to store blocks of code to reuse in other places.
 * 
 */

 pub fn run() {
  greeting("Hello", "Rust");
  
  // Bind the function values to vars.
  
  let get_sum = add(54,1);
  
  println!("Sum of above methond {}", get_sum);
  
  // Closure sum
  // short hand of functions
  let add_sums = |n1:i32, n2:i32| n1+n2;
  
  
  println!("C sum {}", add_sums(3,3));
 }
 
 fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
 }
 
 
 /* 
  ' -> ' is to define the return type from the functions.
 */
 fn add(n1: i32, n2:i32) -> i32 {
  return n1 + n2;
 }