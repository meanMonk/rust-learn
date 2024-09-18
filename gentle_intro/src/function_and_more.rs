
// functions
//  - are one place where the compiler will not work out types for you.

// simple user define fiunction.

fn fn_sqr(x: f64) -> f64 {
    x * x
}

fn fn_abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn fn_factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * fn_factorial(n-1)
    }
}

fn fn_one() {
    let res = fn_sqr(2.0);
    println!("sqr of {} => {}", 2.0, res);
    
    let positive_abs = fn_abs(2.0);
    println!("postive abs {}", positive_abs);
    let negative_abs = fn_abs(-42.0);
    println!("Negative abs {}", negative_abs);
    
    for i in  0..=5 {
        let factorial_5 = fn_factorial(i);
        println!("Factorial of {i} => {factorial_5}"); // named params.
    }
        
}

pub fn fn_more_main() {
    fn_one();
}