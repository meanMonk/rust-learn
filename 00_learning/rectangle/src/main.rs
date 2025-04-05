

// doesn't makes sense both params are related.
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// refactor with tuples. 
// we need to keep in mind .0 = width and .1 is height so hard to remember 
// as we are not conveying so easier to intrudce error.
fn area_tuples(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

// usage of struct.
#[derive(Debug)]
struct  Rectangle  {
    width: u32,
    height: u32,
}

// area calculation with struct of rectangle.

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// Adding useful functionality with derived traits.
// let's talk about the dbg!

// Implementation with impl.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // we can choose to give method name as key of struct. 
    
    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, rect1: &Rectangle) -> bool {
        self.width > rect1.width && self.height > rect1.height
    }
}

fn main() {
    println!("Hello, world!");
    let width = 30;
    let height = 50;
    println!("Area of Rectangle {} sqr pixels!", area(width, height));
    println!("Area Tuples of Rectangle {} sqr pixels!", area_tuples((width, height)));
    
    let rect = Rectangle {
        width : 5,
        height :4
    };
    
    println!("Area of rectangle with Struct {}sqr pixels", area_struct(&rect));
    // println!("Area of rect {}", rect); // fails as struct doesn't implement display.
    println!("Area of rect {:?}", rect); // fails as struct doesn't implement display.
    println!("Area of rect {:#?}", rect); // fails as struct doesn't implement display.
 
 
    // dbg! - ownership of values. works exactly opsite to println! takes reference
    
    let scale = 5;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50
    } ;
    
    dbg!(&rect1);
    // dbg!(rect1);
        
    println!("Area of rectangle with method sytanx {}sqr pixels", rect.area());    
    
    if rect.width() {
        println!("Rectangle has non-zero width {}", rect.width);
    }
    
    println!("rect 1 can hold rect {}", rect1.can_hold(&rect))
}
