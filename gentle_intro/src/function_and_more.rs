
// functions
//  - are one place where the compiler will not work out types for you.

// simple user define fiunction.

use std::f32::consts::PI;

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

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn by_ref_no_deref(x: &i32) -> i32 {
    x + 1
}

// no return type defined as not returning anything.
fn modifies_parent(x: &mut f64) {
    *x = 31.0;
}

/// Reference & Dereferences. 
fn ref_main() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&42);
    let res3 = by_ref_no_deref(&i);
    println!("{i} => {res1} {res2}");
    println!("no deref {i} => {res3} ");
    
    // modifies mut param value.
    let mut float_j = 0.0;
    modifies_parent(&mut float_j);
    println!("modified value of float_j is {float_j}");
    
}


fn fn_cosine() {
    let pi_double =  2.0 * std::f32::consts::PI;
    
    let abs_diff = (pi_double.cos() - 1.0).abs();
    let epsilon = f32::EPSILON;
    println!("abs diff is {abs_diff} {epsilon}");
    assert!(abs_diff <= f32::EPSILON);
    
    let pi: f64 = std::f64::consts::PI;
    let x = pi / 2.0;
    let consine = x.cos();
    println!("cosine of {pi} {consine}");
}


pub fn fn_more_main() {
    fn_one();
    ref_main();
    fn_cosine();
}