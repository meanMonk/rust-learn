use crate::greet;

// Generic Functions!
// We want a function which will dump out any value that implements `Debug`

// so rust needs to inform that T does infact implement `Debug`!

fn dump<T>(value: &T)
where
    T: std::fmt::Debug, // rust knows sensible type of T so can give sensible compiler message.
{
    println!("value is {:?}", value)
}

// as struct doesn't implement Debug
#[derive(Debug)]
struct A {
    name: &'static str,
}

fn learn_generic_fn() {
    let n = 53;
    let f = 23.5;
    let a = A {
        name: "Names of struct",
    };
    dump(&n);
    dump(&f);
    dump(&a);
}

// the operation of squaring number is generic x*x
// Rust generic functions may look a bit overwhelming at first,
// but being explicit means you will know exactly what kind of values you can safely feed it,
// just by looking at the definition.

fn sqr<T>(x: T) -> T::Output
where
    T: std::ops::Mul + Copy,
{
    x * x
}

fn learn_generic_fn_two() {
    println!("Sqr of 2 => {}", sqr(2));
    println!("Sqr of 4.3 => {}", sqr(4.3));
}

pub fn main() {
    greet::greet("Generic Functions!");

    learn_generic_fn();
    learn_generic_fn_two();
}
