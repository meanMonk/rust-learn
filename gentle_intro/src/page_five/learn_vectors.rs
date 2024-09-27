// Vectors:
//  - Vec::new()
//  - vec!
// 
/* 

    // Not all possible methods are available  
    - `push` - push new ele to vect
    - `pop` - remove ele from vec last.
    - `dedupe` - removes duplicates
    
*/

// There's special relationship between Vec<T> and &[T]
// Any method that works on Slices will also directly work on vectors. with explicit use of `as_slices`
// This relation is expressed as `Deref<Target=[T]>`
/* 
    Slices Methods:
        - `first` maybe returns a reference to the first element
        - `last` returns a reference to last element
        - `split_at`
        - `starts_with`
        - `contains` : to check wether a vector contains particular value.
*/

// eg. find the index of value in vec. no inbuild method available.

fn vec_one() {
    let mut v = vec![10,20,30,40,50,60];
    // `&` is used as this is an iterator over references.
    let position_20 = v.iter().position(|x| *x == 20).unwrap();
    println!("position of 20 = {position_20}");
    
    v.extend([22,33,44].iter());
    
    println!("extended vec values {:?}",v);
}

// map over vec and parse content might return result
fn vec_two() {
    
    let nums = ["50", "60","51"];
    let int_arr =  nums.iter().map(|x|x.parse::<i32>());
    let converted: Vec<_> = int_arr.collect();
    
    println!("converted values of array {:?}", converted);
    let nums = ["41","42","43"];
    let int_arr =  nums.iter().map(|x|x.parse::<i32>()); 
    // collect into the single Result
    
    // This is good example of flexibility of `collect`
    
    let converted_r: Result<Vec<_>, _>  = int_arr.collect();
    println!("converted result of vec {:?}", converted_r);
    
}

// `map` : no map method on rust vec as well.


pub fn main() {
    println!("Learning vectors!");
    
    vec_one();
    vec_two();
}