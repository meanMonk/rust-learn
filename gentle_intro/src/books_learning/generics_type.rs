// Generic Data Types.
// we use generics to create definitions for items like function signature, structs
// let's define the "function, struct, enums and methods" using generics.

// In Function Definitions
// we place the generics in the signature of the fuction where we would usually specify the data types of the params and returnva lues. 
// doing so make code more flexible and provider more functionality to callers of our funtion while preventing code duplication.

// Continuing with our `largest` function.

// ex. large of i32.
fn laregest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest =  &list[0];
    
    for item in list {
        if item >  largest {
            largest =  item;
        }
    }
    
    largest
}

fn run_largest_one() {
    let number_list = vec![42,50,43,54,20];
    let result = laregest_i32(&number_list);
    
    println!("largest from number list {result}");
    
    let char_list = vec!['y','a','m'];
    
    let c_result = largest_char(&char_list);
    println!("largest from number char {c_result}");
    
}

// so now the largest_i32 & largest_char has same code so let's eliminate the duplication by using generic type.
// Rust type naming convention is `UpperCamelCase` short for type `T` is the default choice of most rust programs.
// when we use par in body of function we have to declare the param name in the signature so compiler knows what that means.
// when we use a type parameter name in function signature, we have to declare the type parameter name before we use it.
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn run_largest_generic() {
    let number_list = vec![34, 50, 25, 100,64];
    
    let result = largest_generic(&number_list);
    
    println!("largest from number list {result}");
    
    let char_list = vec!['v', 'a', 'n'];
    
    let result = largest_generic(&char_list);
    
    println!("largest from char list {result}");
    
}


// Generics in Strucrt Definitions.
// we can also define the struct to use a generic type parameter in one or more fields using the <> syntax.
// 

struct  Point<T> {
    x: T,
    y: T
}

// to have different type for each param we can pass multiple generic. 

struct MultiPoint<T, U> {
    x: T, 
    y: U
}

fn struct_one() {
    let integer = Point { x: 5, y: 10};
    let float = Point { x: 5.3, y: 10.2};
    
    println!("integer point x = {}", integer.get_x());
    println!("float point x = {}", float.get_x());
    
    let int_and_float = MultiPoint { x : 5, y : 4.2};
}


// Use of generics in Enum definistions. 
// by using the generic type we can avoid the duplications.
enum ResultType<T, E> {
    Ok(T),
    Err(E)
}


// Generic in Method Definition.
//  we can implement mthods on structs and enums and use generic types in their definitions too.
// we’ve defined a method named x on Point<T> that returns a reference to the data in the field x.
impl <T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}


// Note: performance of code using generics.
//  - Using generic types won't make your program run any slower than it would with concrete types.
// Rust accomplishses this by performaing "monomorphization" of the code using generics at compile time.
// "Monomorphization" 
//   - is the process of turning generic code into specific code by filling in the concrete types that are used when compiled
// 

// Option<T> is enum.

pub fn main() {
    run_largest_one();
    run_largest_generic();
    struct_one();
}