// Arrays - Fixd list where elements are the same data types.

use std::mem;

pub fn run() {
  let mut numbers: [i32; 5]= [1,2,3,4,5];
 
  // get array
  println!("{:?}", numbers);
  
  // get single val
  println!("single value of array : {}", numbers[2]);
  
  numbers[2] = 8;
  
  println!("{:?}", numbers[2]);
  
  // array length 
  println!("Array length : {}", numbers.len());
  
  // array are stack allocated
  println!("Array occupies: {} bytes", mem::size_of_val(&numbers));
  
  // get slice 
  // &numbers -- is nothing but setting refers to numbers var
  let slice: &[i32] = &numbers[0..3];
  
  println!("Slice: {:?}", slice);
  
}