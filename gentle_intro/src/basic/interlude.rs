// Interlude:
// `std::env:args` is how you access command line arguments.


fn env_one() {
    for param in std::env::args() {
        println!("arguments = {param}");
    }
    
    // with collect we can skip and conver it to vectors Vec.
    // let args = std::env::args().skip(1).collect();
    
    
}

pub fn interlude_main(){
    println!("!! Interlude Program !!");
    
    env_one();
}