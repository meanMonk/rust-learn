// enums used to enumerate all possible variants.

// defining enum of ip addres kind.
// variants of the enum are namespaced under it's identifier and 
// we use a double colon to separate the two.
#[derive(Debug)]
enum IpAddrKind {
    V4, 
    V6
}

enum Message {
    Quit, 
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

// Destructuring structs

struct  Point {
    x: i32, 
    y: i32
}

// Rust does not have null but have enum is Option<T>
// enum Option<T> {
//     None, 
//     Some(T)
// }


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, 
        Some(i) => Some(i + 1)
    }
}

fn main() {
    println!("Hello - ENUMS!!");
    
    // enum values. instance of two variants of IpAddrKind.
    let four = IpAddrKind::V4;
    let six =IpAddrKind::V6;
    
    println!("Enum value of four {:?} six {:?}", four, six);
    
    // matching and using of Enum. 
    
    let x = Some(5);
    let y = plus_one(x);
    let znone = plus_one(None);
    
    println!("matching info y {} x {} znone {znone:?}", x.unwrap(), y.unwrap());
    
    // pattern matching
    // Matching named variables.
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {y:?}"),
        _ => println!("Got Default Case x {x:?}"),
    }
    
    println!("at the end: x = {x:?}, y={y}");
    // Matching multiple patterns
    
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // matching ranges with values.
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }
    
    let x = 'c';
    
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Something else"),
    }
    
    let p = Point {x:0, y:7};
    let Point {x:a, y:b} = p;
    assert_eq!(7,b);
    assert_eq!(0,a);
    
    
    let msg =  Message::Write(String::from("0,160,255"));
    
    match msg {
        Message::Quit => println!("The quit variant has no data to destructure"),
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
    
    // Destructuring Enum
}
