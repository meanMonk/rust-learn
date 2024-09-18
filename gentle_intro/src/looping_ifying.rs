/// Looping and Ifying.

fn for_one() {
    for i in 0..5{
        println!("simple loop {}", i);
    }
}

fn for_two() {
    for i in 0..5 {
        if i %2 ==0 {
            println!("{} EVEN Index!", i);
        } else {
            println!("{} ODD Index!", i);
        }
    }
}

// can be written as!
fn for_three() {
    for i in 0..5  {
        let even_odd = if i % 2== 0 {"Even"} else {"Odd"};        
        println!("{} - {}", i, even_odd);
    }
    
    // NOTE: there aren't any semicolons in block which means it will return that line.
}

/// Adding things Up. 

fn add_one() {
    // Note: Immutable - a variable that cannot vary?
    // `let` var default can be assigned a value while declaring.
    // `mut` keyword will make var to be mutable so we can update again.
    
    let mut sum = 0;
    for i in 0..5  {
        sum += i ;  // (+=) is trait name as `AddAssign` and like wise other operators too.
    }
    
    println!("Sum of numbers 0 to 4 = {}", sum);
}


// casting value to float to make program running.
fn add_two() {
    let mut sum = 0.0;
    for i in 0..5  {
        sum += i as f64
    }
    println!("Sum of numbers with float  {}", sum);
}


pub fn loop_main() {
    for_one();
    for_two();
    for_three();
    add_one();
    add_two();
}

