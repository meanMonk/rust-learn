// Tuples.
// useful to return multiple values form function

fn add_mul(x: f64,y:f64) -> (f64,f64) {
    (x+y, x*y)
}

pub fn main() {
    let t =  add_mul(2.0, 4.0);
    
    // can print with debug
    println!("value t {:?}", t);
    
    // can `index` the values.
    println!("add {} mul {}", t.0, t.1);
    
    // can extract values
    let (add, mul) = t;
    println!("add => {add}, mul => {mul}");
    
    // tuples may contain different values
    
    let tuple = ("hello", 5, 'c');
    
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
    
    // Tuples also return by enumerate method of Iterators.
    
    for t in ["zero", "one", "two"].iter().enumerate() {
        println!(" iterators =>  {:?} {} {} ", t, t.0, t.1);
    }
}
