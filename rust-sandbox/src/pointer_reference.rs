/***
 *  // Reference & pointers 
 *  ---  Point to resources in memory.
 * 
 */

 pub fn run() {
   println!("!!FROM POINTER AND REF!!");
   
   // Primitive array.
   
   let arr1 = [1,2,3,4];
   
   let arr2 = arr1;
   
   println!("Values {:?}",(arr1, arr2));
   
   
   // with non-primities, if you assign another variable to peice of data, the first variable will no longer hold that value , you will need to use reference to point to that resource.
   
   // Vector.
   let vec1 = vec![1,2,3,4];
   
   let vec2 = &vec1;
   
   println!("Values of Vector {:?}",(&vec1, vec2));
 }