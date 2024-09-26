use crate::greet;

// Simple Enum
// Enums are types which have few definite values.
// a direction have only 4 values.

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// enum can have method defines on them just like structs
// The `match` expression is basic way to handle `enum` values
// These Enum values can't be compared.

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }

    // no perticular ordering here
    // here method defines successor of each define value.

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

fn learn_enum() {
    let first = Direction::Up;

    println!("first direction {}", first.as_str());

    let mut d = first;

    for _ in 0..8 {
        println!("d {:?}", d);
        d = d.next();
    }
}

// These enums do have a natural ordering, but you have to ask nicely.
// After placing `#[derive(PartialEq,PartialOrd)]` in front of enum Speed,
// then it's indeed true that `Speed::Fast > Speed::Slow` and `Speed::Medium != Speed::Slow`.
#[derive(PartialEq, PartialOrd)]
enum Difficulty {
    Easy = 1,
    Medium,
    Hard,
}
// `name` is too vague we can't called the term here is `variant`

// Enum in their full glory.
#[derive(Debug)]
enum Value {
    Number(f64),
    Bool(bool),
    Str(String),
}

impl Value {
    fn to_str(self) -> Option<String> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

// Again, this enum can only contain one of these values; its size will be the size of the largest variant.
// they also know how what kind of value they contain that is superpower of `match`.

fn eat_and_dump(v: Value) {
    use Value::*;

    match v {
        Number(n) => println!("Number is {}", n),
        Str(s) => println!("Str is {}", s),
        Bool(b) => println!("Bool is {}", b),
    }
}

fn learn_enum_two() {
    use Value::*;
    let n = Number(2.4);
    let s = Str("hello".to_string());
    let b = Bool(true);

    println!("n {:?} s {:?} b {:?}", n, s, b);

    println!("value of s {:?}", s.to_str());

    let s = Str("hello world".to_string());
    eat_and_dump(n);
    eat_and_dump(s);
    eat_and_dump(b);
}

pub fn main() {
    greet::greet("Learn Enums!");
    learn_enum();
    learn_enum_two();
}
