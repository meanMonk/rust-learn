// Matching
// In case of multigiual we can write it with match like below.

/* 
     match multilingual.find('п') {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);
        },
        None => println!("couldn't find the greeting, Товарищ")
    };
    
    
    // match consists of several patterns with a matching value following the fat arrow, 
    // separated by commas. 
    // It has conveniently unwrapped the value from the Option and bound it to idx. 
    // You must specify all the possibilities, so we have to handle None.

*/


/* 
    // match can also act like C- switch statement.
    
    let number = match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Many", // _ is like default in c
    }
    
*/


/* 
    // Rust match statements can also match on ranges. 
    // @Note that these ranges have three dots and are inclusive ranges, 
    // so that the first condition would match 3.


    let text =  match n {
    
        0...3 => "small", 
        4...6 => "medium",
        _ => "large",
    }

*/

pub fn matching_main() {
    println!("!! MATCHING !!");
}