// Vectors - Vectors are resizable array.

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32>= vec![1,2,3,4,5];
 
  // get Vector
  println!("{:?}", numbers);
  
  // Add on to vector
  numbers.push(6);
  numbers.push(7);
  // Pop on last values
  println!("{:?}", numbers);
  
  numbers.pop();
  
  println!("{:?}", numbers);
  
  // get single val
  println!("single value of Vector : {}", numbers[2]);
  
  numbers[2] = 8;
  
  println!("{:?}", numbers[2]);
  
  // Vector length 
  println!("Vector length : {}", numbers.len());
  
  // Vector are stack allocated
  println!("Vector occupies: {} bytes", mem::size_of_val(&numbers));
  
  // get slice 
  // &numbers -- is nothing but setting refers to numbers var
  let slice: &[i32] = &numbers[0..3];
  
  println!("Slice: {:?}", slice);
  
  // loop through the numbers values.
  
  for x in numbers.iter() {
    println!("Number: {}", x);
  }
  
  // loop and mutate values
  
  for x in numbers.iter_mut() {
    *x *=2;
  }
  
  println!(" mutated numbers vec : {:?}", numbers);
  
  
}