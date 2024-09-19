/// # Arrays & Slices.
/// array are values packed nose to tail in memory.
/// Arrays are indexed from zero.
/// Arrays are fixed in size,
/// They can be mutable but you can not add new elements to it.

fn array_one() {
    let arr = [10,20,30,40,50];
    // As arrays are not used that often in Rust, because the type of an array includes its size
    let ele_first = arr[0];
    println!("first {ele_first}");
    
    // if increase index return out of bound.
    for i in 0..5  {
        println!("[{i}] {}",arr[i]);
    }
    println!("length {}",arr.len());
}


// focus on &[i32] Rust slices keep track of their size (and will panic if you try to access outside that size) 
// and you have to explicitly say that you want to pass an array as a slice using the & operator.

fn sum_of_array(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

// read as slice of i32
fn array_two() {
    let arr = [10,20,30,40];
    let sum = sum_of_array(&arr);
    println!("Sum {} Array elements {:?}", sum, arr);
}

// `{}` won't work to print out the arr we need to use {:?} - debug print.
fn array_slicing_dicing() {
    let ints= [1,2,3,5];
    let floats = [1.3, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1,2],[4,5]]; // arrays of array no problem but imp that an array contains values of only one type.
    
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
    
    // slices give different view of same array.
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];
    println!("slice1 {:?}", slice1);
    println!("slice1 {:?}", slice2);
    
    // Note: A copy of the data is never made. 
    // These slices all borrow their data from their arrays. They have a very intimate relationship with that array
    
}

pub fn array_slices_main() {
    array_one();
    array_two();
    array_slicing_dicing()
}
