// to disable the warning of dead_code
#![allow(dead_code)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

#[derive(Debug)] 
struct Rectangle {
    width: u32, 
    height: u32,
}


impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// greet function test for custom message in tests.

pub fn greet(name:&str) -> String {
    format!("Hello {name}")
}

// guess game to test the should_panic macro.

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be less than and equal to 100, got {value}");
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {value}");
        }
        
        Guess { value }
    }
}

// something that function output.

pub fn prints_and_returns_10(a: i32) -> i32{
    println!("I got the value of {a}");
    10
}
// testing the private functions.

fn internal_add_three(value: i32) -> i32 {
    value + 3
}


// this annotation will tail the cargo to run tests only when `cargo test` not when we run `cargo build`
// cfg - stands for configuration - tells that following items should only be included given in certain configuration option.

#[cfg(test)]
mod adder_tests {
    use super::*;

    #[test]
    fn exploration_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    // #[test]
    // fn failed_test() {
    //     panic!("test is going to failed!");
    // }
    
    // Checking Results with the assert! Macro
    #[test]
    fn larger_can_hold_smallet() {
        let small = Rectangle {
            width: 5, 
            height: 6
        };
        let large = Rectangle {
            width: 10, 
            height: 15
        };
        
        assert!(large.can_hold(&small));
    }
    
    #[test]
    fn smaller_can_not_hold_larger() {
        let small = Rectangle {
            width: 5, 
            height: 6
        };
        let large = Rectangle {
            width: 10, 
            height: 15
        };
        
        assert!(!small.can_hold(&large));
    }
    
    // Testing equality with assert_eq! & assert_ne! macros
    #[test]
    fn test_add_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_add_two_failed() {
        let result = add_two(2);
        assert_ne!(result, 5);
    }
    
    // Adding custom failure messages.
    // assert! assert_eq! assert_ne!
    // Custom messages are useful for documenting what an assertion means; 
    // when a test fails, you’ll have a better idea of what the problem is with the code.
    
    #[test]
    fn greeting_contains_name() {
        let result = greet("Rust");
        assert!(result.contains("Rust"));
    }
    
    // #[test]
    // fn greeting_contains_failure() {
    //     let result = greet("Rust");
    //     assert!(result.contains("Failure"), 
    //     "Greeting does not contains name, value was {result}");
    // }
    
    
    // checking for panic with `should_panic`
    // In addition to checking return values, it’s important to check that our code handles error conditions as we expect
    
    #[test]    
    #[should_panic]
    fn under_range() {
        Guess::new(99);
    }
    
    #[test]    
    #[should_panic]
    fn greater_than_100() {
        Guess::new(199);
    }
    
    // panic with expected messages.
    #[test]
    #[should_panic(expected="less than and equal to 100")]
    fn less_than_1() {
        Guess::new(-1);
    }
    
    // using of Result<T,E> in tests.
    #[test]
    fn does_it_works() -> Result<(), String> {
        let result =  add(2,2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two + two does not equal to four!"))
        }
    }
    
    #[test]
    fn this_test_will_pass(){ 
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }
    
    #[test]
    fn this_test_will_fail(){ 
        let value = prints_and_returns_10(4);
        assert_eq!(value, 5);
    }
    
    // running tests by subsets of name
    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn add_two_and_three() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }
    
    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
    // ignoring the test unless requested specifically.
    
    #[test]
    #[ignore]
    fn ignored_test_add_two_and_five() {
        let result = add_two(5);
        assert_eq!(result, 7)
    }
    
    // testing the internal private functions, 
    #[test]
    fn test_internal_add(){
        let result =  internal_add_three(4);
        assert_eq!(result, 7);
    }
    
 }

 
//  running single test with nameca
// `cargo test one_hundred`

// Filtering to run the multiple tests.
// `cargo test add`