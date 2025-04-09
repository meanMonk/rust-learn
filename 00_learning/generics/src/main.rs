// fn find_largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest =  item;
//         }
//     }
    
//     largest
// }

// fn find_largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
    
//     for char in list {
//         if char > largest {
//             largest = char;
//         }
//     }
    
//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut result = &list[0];
    
    for item in list {
        if item > result {
            result = item;
        }
    }
    
    result
}

// generics in struct defination.
#[derive(Debug)]
struct Point<T> {
    x: T, 
    y: T
}

// in methods definition
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// struct with multiple generics values. 
#[derive(Debug)]
struct ComplexPoint<T, U> {
    x: T, 
    y: U
}


struct MixPoint<X1, Y1> {
    x: X1, 
    y: Y1,
}

impl<X1,Y1> MixPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixPoint<X2,Y2>) -> MixPoint<X1, Y2> {
        MixPoint {
            x: self.x,
            y: other.y
        }
    }
}

// use of generics in Enum defination .
#[derive(Debug)]
enum Result<T,E> {
    Ok(T),
    Err(E)
}

fn main() {
    // Generics is tool to remove duplication of code. 
    let number_list =  vec![12,33,24,45,100,65];
    
    println!("The largest number is {}", largest(&number_list));
    
    // second set of numbers
    let number_list =  vec![100, 102,104,124,145,123];
    
    println!("The largest number in second set {}", largest(&number_list));
    
    // impl function that find largest of i32 and char. 
    
    let char_list = vec!['a','b','c','d','e','f','g','h'];
    
    println!("The largest char in list {}",largest(&char_list));
    
    // implement the same function with generics.
    println!("The largest of number with generics {}", largest(&number_list));
    println!("The largest of char with generics {}", largest(&char_list));
    
    // in struct defines.
    let integer = Point {x: 9, y: 10};
    let float = Point {x: 2.9, y: 1.0};
    
    println!("structs for int & float {integer:?} - {float:?}");
    
    let mx_point = ComplexPoint{x: 9, y:8.89};
    println!("structs for int & float {mx_point:?}");
    
    // usage of generics in mix points..
    let p1 = MixPoint { x: 5, y: 10.4 };
    let p2 = MixPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    
    
}


/* 
 // Summary, here are the steps we tool to change the code.
 
  - identify duplicate code 
  - extract the duplicate code into the body of the function, and specify the inputs & return values of that code in the function signature.
  - update the two instances of duplicated code to call the function instead

//   now we will use the same steps with genercis to reduce code duplication. 
- in same way that the function body can operate on an abstract list instead of specific values
- generics allow code to operate on abstract types.
  
*/


// Generics ussages on performance of code." Monomorphization"
// Monomorphization is the process of turning generic code into specific code by filling concrete 
// types that are used when compiled.
