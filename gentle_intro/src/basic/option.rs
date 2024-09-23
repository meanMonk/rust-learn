// Option : You can think of Option as box which may contain a value or nothing (None)
// It is very common for Rust functions/methods to return such maybe-boxes
// Every option is either Some or Contains a value or None and does not.
// Option types are very common in Rust code.
/*  Uses :
     - Initial values
     - Return values for functions that are not defined over their entire input.
     - Return value for reporting simple errors, where None is return on error.
     - Option structur fields
     - Struct field that can be loaned or taken
     - Optional function arguments
     - Nullable pointers
     - Swapping things out of difficult situations.
*/
// Options are commonly paired with pattern matching to query the presense of value and take action.
//
/* 
    If you were to unwrap last, you would get a panic. 
    But at least you can call is_some first to make sure - for instance, 
    if you had a distinct no-value default:
    
    so the shortcut is to `option.unwrap_or` return value given if Option is None.
    get returns reference so you have to make up &i32./..
    
    let last = *slice.get(5).unwrap_or(&-1);

*/



// Addition of ppatern matching Option provides a wide variety of different methods.
/*
    - is_some, is_none,
    - as_ref as_mut as_deref as_deref_mut as_pin_ref as_pin_mut
    - extracting the contain values.
        - expect | unwrap | unwrap_or | unwrap_or_default | unwrap_or_else
    - Transforming contained values.
        - ok_or | ok_or_else | transpose
*/

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn check_optional(optional: Option<Box<i32>>) {
    match optional {
        Some(p) => println!("have value {p}!"),
        None => println!("has no value!"),
    }
}

/**

fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    let a = stack.pop();
    let b = stack.pop();

    match (a, b) {
        (Some(x), Some(y)) => Some(x+y),
        _ => None,
    }
}
    can be written as.

fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    Some(stack.pop()? + stack.pop()?);
}

*/

fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    Some(stack.pop()? + stack.pop()?)
}

pub fn option_main() {
    let result = divide(2.0, 3.0);

    match result {
        Some(x) => println!("Result : {x}"),
        None => println!("Cannot divide by 0"),
    }

    let result1 = divide(2.0, 0.0);

    match result1 {
        Some(x) => println!("Result : {x}"),
        None => println!("Cannot divide by 0"),
    }

    // Options and Pointers.
    // Rust pointers must alway point to valid location, no null referernce

    let optional = None;
    check_optional(optional);
    let optional = Some(Box::new(300));
    
    check_optional(optional);

    // The question mark operator `?`.
    // similar to Result type, handling Some / None can be tedious.
    // `?` operator hides some of the boilerplate of propogating value up the  call stack.

    let mut vec = vec![12, 52];
    let res = add_last_numbers(&mut vec);
    println!("Add of numbers {:?}", res);
}
