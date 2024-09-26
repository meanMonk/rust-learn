use crate::greet;

// Closures
// A great deal of Rust power comes from closures
// In their simplest form they act like shortcut.
// |x| x * x  (this is nothing but the closure function)
// Define as:
// NOTE:
//  - Closure is a struct
//  - Borrows values from it's environment. that therefore it has lifetime.

// NOTE: All closure are unique types, but they have traits in common. so even though we don't know the exact type, we know the generic constraint.

fn closure_one() {
    let f = |x| x * x;

    for i in 19..23 {
        println!("sqr of {i} => {}", f(i));
    }

    // So, the first call fixes the type of the argument x. It's equivalent to this function:
    /*
       fn f (x:i32) -> i32{
           x * x
       }
    */
    // Big difference between functions & closures apart from need of explicity typing

    let m = 2.0;
    let c = 1.0;

    let lin = |x| m * x + c;

    println!("res {} {}", lin(1.0), lin(2.0));
    // You cannot do this with the explicit fn form -
    // it does not know about variables in the enclosing scope.
    // The closure has borrowed m and c from its context.
}

// Under the hood closure is `struct` that is callable (implements the call operator)
// It behaves as if it was written out like this.

/*

// here is anonymous closure
// compiler turn simple closure into all code below.

let MyAnonymousClosure1 = |x| m*x + c;

// ############# OR ############

struct MyAnonymousClosure1<'a> {
    m: &'a f64,
    c: &'a f64,
}
impl <'a> MyAnonymousClosure1<'a> {
    fn call(&self, x:f64) -> f64 {
        self.m * x + self.c
    }
}

*/

// implement apply function.

// so now apply works for any type T such that T implements Fn(f64) -> f64
// which is the funciton which takes and returns f64

fn apply<F>(x: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    f(x)
}

fn closure_two() {
    let m = 2.0;
    let c = 1.0;
    let lin = |x| m * x + c;

    println!("out put of lin {}", lin(2.9));
    let res1 = apply(2.9, lin);
    println!("res1 with apply using lin {}", res1);

    println!("O/P after use with apply {}", lin(2.5));

    let res2 = apply(3.14, |x| x.sin());
    println!("res2 for sin of {}", res2);
}

/*
    Calling a closure is method call:
    The 3 kinds of function traits correspond to the 3 kinds of methods.

    - `Fn` struct passed as `&self`
    - `FnMut` struct passed as `&mut self`
    - `FnOnce` struct passed as `self`
*/

// so it's possible for closure to mutate it's captured reference.

fn mutate<F>(mut f: F)
where
    F: FnMut(),
{
    f()
}

fn closure_three() {
    let mut s = "world";

    mutate(|| s = "Hello");

    assert_eq!(s, "Hello");
}

// can not escape the rule of borrowing.

fn closure_four() {
    let mut s = "world";

    let mut changer = || s = "hello";

    changer();

    println!("value of s {}", s);
    assert_eq!(s, "hello");
}

// sometime you don't want a closure to borrow those variable but instead move them.

fn closure_five() {
    let name = "dolly".to_string();

    let age = 43;

    // this `move` moves the value of variable to closure and no more accessible outside.
    let c = move || {
        println!("name {name} age {age}");
    };

    c();

    // this will fail as values has been moved to closure.
    // println!("value of name after move {name}");
}

// to fix the above issue we can make use of clone copy to move into the closure.

fn closure_six() {
    let name = "Zoom".to_string();
    let app = "video calling".to_string();

    let cname = name.to_string();

    let prepare = move || {
        println!("{app} - {cname}");
    };

    prepare();
    // name is available after move of copy.
    println!("Application Name  {name}");
}

// why do we need moved closures?
// - we might need to call them at the point where the original context no logner exists.
// A classic case is when creating the thread.
// A moved closure does not borrow, so doesn't have a lifetime

// Major use of closure in iterator methods consider `range` method
// to operate on any other iterator using closure is straightforward

// working back on traits.

struct FRange {
    val: f64,
    end: f64,
    incr: f64,
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {
        val: x1,
        end: x2,
        incr: skip,
    }
}

// trait iterator
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

fn closure_seven() {
    let sum: f64 = range(0.0, 1.0, 0.1).map(|x| x.sin()).sum();
    println!("some of values {sum}");
}

// filter is also another usefule method.

fn closure_eight() {
    let tuples = [(10, "ten"), (20, "twenty"), (30, "thirty"), (40, "fourty")];
    let iter = tuples.iter().filter(|t| t.0 > 10).map(|t| t.1);

    for name in iter {
        println!("{name}");
    }
}

pub fn main() {
    greet::greet("Learn Closures In Rust");

    closure_one();
    closure_two();
    closure_three();
    closure_four();
    closure_five();
    closure_six();
    closure_seven();
    closure_eight();
}
