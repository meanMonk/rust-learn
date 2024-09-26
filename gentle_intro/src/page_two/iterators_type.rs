// The Three Kinds of Iterators.
// 3 kinds corresponds to the three basci argument types.
// Assume we have vector of `String` values
// Below are explicit & implicit iterator types with value return by the iterator.

// better and simple to remember.
/* 
    // Explicity iterator
    
    for s in vec.iter() {} // &String
    for s in vec.iter_mut() {} // &mut String
    for s in vec.into_iter() {} // String
    
*/

// - `into_iter()` consumes the vector and extracts it's strings and so afterwards the vector is no longer available.
// it has been moved. used to saying`for s in vec`!


/* 
    // Implicit iterator
    
    for s in &vec {} // &String
    for s in &mut vec {}  // &mut String
    for s in vec {} // String 
*/



/* 
    It's important to understand how the three kinds works because 
    Rust relies heavily on type deduction - you won't often see explicit types in closure arguments. 
    And this is a Good Thing, because it would be noisy 
    if all those types were explicitly typed out.
    However, the price of this compact code is that you need to know what the implicit types actually are!
*/

/* 

    - `map` takes whatever value the iterator returns and converts it into something else, 
    - `filter` takes a reference to that value. 
    
    In this case, we're using iter so the iterator item type is &String.
    Note that filter receives a reference to this type. 
    
*/


/* 
    for n in vec.iter().map(|x: &String| x.len()) {....} n is usize
    
    // 
    for s in vec.iter().filter(|x: &&String| x.len()) {...} s is &string
*/