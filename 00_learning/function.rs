
// functions are one where compilers will not work out types for you.

fn sqr(x : f64) -> f64 {
    x * x
    // or we can write. 
    // return x * x;
}


// absolute value of fn. 

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// number always clam

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}


// factorial of numbers. 


fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

// Values can also be passed by reference. A reference is created by & and dereferenced by *.
// 
fn by_ref(x: &i32) -> i32{
    *x + 1
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

// array as input with slices and prepare sum.

fn sum_of_array(values: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    
    for i in 0..values.len() {
        sum += values[i]
    }
    
    sum
}

fn main() {
    let sq = sqr(2.0);
    println!("square is {} ", sq);
    let abs_n = abs(2.0);
    println!("absolute is {} ", abs_n);
    let clamp_n = clamp(2.0, 3.0, 12.0);
    println!("clamp n is {} ", clamp_n);
    let clamp_m = clamp(13.0, 3.0, 12.0);
    println!("clamp m is {} ", clamp_m);
    println!("factorial of {} is {} ", 4, factorial(4));
    println!("factorial of {} is {} ", 6, factorial(6));
    println!("factorial of {} is {} ", 5, factorial(5));
    
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1,res2);
    
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
    
    let pi_x: f64 = 2.0 * std::f64::consts::PI;
    let abs_difference = (pi_x.cos() - 1.0).abs();
    println!("abs_difference is {} {}", pi_x, abs_difference);

    assert!(abs_difference < 1e-10);
    
    // Arrays and Slices
    // All statically-typed languages have arrays, which are values packed nose to tail in memory. Arrays are indexed from zero:

    // Array.
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }
    println!("length {}", arr.len());
    
    // invocation of array.
    let arr = [11,22,33,44];
    // look at that &
    let res = sum_of_array(&arr);
    println!("sum {}", res);
}