// Vectors:
// these are re-sizable arrays
// Rust type `Vec` pronounced 'vector' behave very much like slice in fact;
// It must be declared as mutable.  `let mut v = Vec::new()`;

// There is useful little macro `vec!` for initializing a vector.
// You can remove the values from end of vector using `pop` and exetend vector with any compatible iterator.

// Vectors compare with each other and with slices by value.

// Vectors can be sort and removed duplicateds.

fn insert_remove_fn() {
    let mut v1 = vec![90,91,93,94,97,95];
    
    println!("Before insert remove {:?}", v1);
    v1.insert(2, 92);
    println!("after insert remove {:?}", v1);
    v1.remove(5);
    println!("after remove {:?}", v1);

    
    
    let mut v2 = vec![-5,-1,0,0,6,2,4,5];
    
    println!("Vec before sort {:?}", v2);
    v2.sort(); // to sort the vector elements
    println!("Vec After sort {:?}", v2);
    v2.dedup(); // to remove duplicated element from vec.
    println!("Vec After dedupe {:?}", v2);
    
}

fn vec_extend() {
    let mut v1 = vec![97,98,99,100,101];
    v1.pop();
    
    let mut v2 = Vec::new();
    
    v2.push(94);
    v2.push(95);
    v2.push(96);
    
    v2.extend(0..2);
    println!("printing Vec v2 {:?}", v2);
    v2.extend(v1);
    println!("printing Vec v2 {:?}", v2);
}



fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn vector_one() {
    let mut v = Vec::new();
    v.push(200);
    v.push(300);
    v.push(400);

    let first = v[0];
    let maybe_first = v.get(0);

    println!("v is {:?} {:#?}", v, v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);

    // :NOTE so-important borrow operator `&` is coercing the vector into a slice
    dump(&v);

    let slice = &v[1..];
    println!("Slice is {:?}", slice);
}



pub fn vec_main() {
    vector_one();
    vec_extend();
    insert_remove_fn();
}
