#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    // matching over a reference by adding `&`
    match &optional_point {
        Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }
    
    // also we can do matching over the Option not &Option.
    // Some(ref p) => ....

    println!("{optional_point:?}"); // Don't change this line.
}
