// Rust is curly brackets lang with semicolons; c++ style comments & main function.
// ! mark indicates that it's micro call.


fn main() {
    let answer = 43;
    // println! & assert_eq! are micros in rust. can take format strings! 
    println!("Hello world {}", answer);
    assert_eq!(43, answer);
    
    // for loop 
    println!("🤩 {:|^30}", " LOOPING & IFYING ");
    
    for i in 0..5 {
        println!("Namaste at {}",i);
        
        if i%2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
    
    
    for i in 0..6 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} is => {}" ,i,even_odd );
    }
    
    // add 4
    // `mut` - adding the magic word mut make this variable mutable.
    let mut sum = 0;
    
    for i in 0..5 {
        sum += i;
    }
    
    println!("sum of 0 ... 5 = {}", sum);
    
    // add 5
    let mut sum = 0.0;
    
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum of 0 ... 5 float = {}", sum);
}