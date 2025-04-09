fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest =  item;
        }
    }
    
    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    
    for char in list {
        if char > largest {
            largest = char;
        }
    }
    
    largest
}

fn largest<T: Ordering>(list: &[T]) -> &T {
    let mut result = &list[0];
    
    for item in list {
        if item > result {
            result = item;
        }
    }
    
    result
}

fn main() {
    // Generics is tool to remove duplication of code. 
    let number_list =  vec![12,33,24,45,100,65];
    
    println!("The largest number is {}", find_largest(&number_list));
    
    // second set of numbers
    let number_list =  vec![100, 102,104,124,145,123];
    
    println!("The largest number in second set {}", find_largest(&number_list));
    
    // impl function that find largest of i32 and char. 
    
    let char_list = vec!['a','b','c','d','e','f','g','h'];
    
    let largest = find_largest_char(&char_list);
    
    println!("The largest char in list {largest}");
    
    // implement the same function with generics.
    println!("The largest of number with generics {}", largest(&number_list));
    println!("The largest of char with generics {}", largest(&char_list));
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
