// Traits.
// Rust structs cannot inherit from other structs; they are all unique types.
// There is no sub-typing. they are dumb data.

// `Traits` come into the picture to established relationship between types.

use core::fmt;

// we have added new methods `show` to i32 & f64
trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn trait_learn() {
    let answer = 42;
    let answer_s = answer.show();
    println!("Answer {answer}");
    println!("Show : {answer_s}");
    
    let float = 5.45;
    let float_s = float.show();
    println!("Float {float}");
    println!("Show: {float_s}");
}


// learning basic traits of the standard library (they tend to hunt in packs)

// as traits has been defined no more needed!
// #[derive(Debug)] 
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(name: &str, l_name: &str) -> Self {
        Person {
            first_name: name.to_string(),
            last_name: l_name.to_string(),
        }
    }
    
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// use of standar lib trait to implement fmt.

impl fmt::Debug for Person {
    // `write!` is useful macro
    // `f` is anything that implements `Write`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

fn person_with_traits() {
    let person = Person::new("sahil", "Learning Rust");
    
    println!("Welcome {:?}", person); // formatter added which ouput.
    // Welcome sahil Learning Rust
}

// ITERATOR OVER FLOATING POINT RANGE.
// we know range before `(0..n)` but they don't work for floating point.
// Iterator is struct with next method.

struct FRange {
    val: f64,
    end: f64, 
    incr: f64
}

fn range(x1: f64,x2: f64,skip: f64) -> FRange {
    FRange { val: x1, end: x2, incr: skip }
}

impl Iterator for FRange {
    type Item = f64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}

fn range_learn() {
    for x in range(0.0,1.0, 0.1) {
        println!("{:.2} ", x);
    }
    // as impl iterator we other mthods available.
    // so can map and collect them into vector.
    
    let v_range: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.abs()).collect();
    
    println!("vec of range {:?}", v_range);
}


pub fn main() {
    trait_learn();   
    person_with_traits();
    range_learn();
}