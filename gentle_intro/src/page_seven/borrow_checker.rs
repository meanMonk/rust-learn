// Changing the unchangeable.
// you wonder if it's ever possible to get around the restriction of the borrow checker.

use std::cell::{Cell, RefCell};

// changing the value with out even making the variable mutable.
// so this is perfect safe since value inside the cell only accessed through `set` and `get`.
// This called as INTERIOR MUTABILITY
// The other one is called "INHERITED MUTABILITY" `let mut answer = 23;`

// `Cell` we can change the value contained within them with `set` even if the cell itself is not mutable.
// `Cell` only works with `Copy` types.

fn cell_one() {
    let answer = Cell::new(42);
    
    println!("value of answer before change {}", answer.get());
    
    answer.set(200);
    // changing the value of answer even if the var is not mutable

    println!("value of answer after change {}", answer.get());    
}


// for other values we have to get reference we can work on, either mutable or immutable.
// `RefCell` provides this feature. you can ask reference to contained value explicitly.

// greeting not declared as mutable.

fn ref_cell_one() {
    
    let greeting = RefCell::new("hello".to_string());
    
    // borrowing value and comparing.
    // .borrow() > returns Ref<string> so we need to deref with `*` to compare
    // .borrow().len() -> this defer implicitly so no need to use `*`
    assert_eq!(*greeting.borrow(), "hello");
    assert_eq!(greeting.borrow().len(), 5);
    
    *greeting.borrow_mut() = "hola".to_string();
    
    assert_ne!(*greeting.borrow(), "hello");
}
// using RefCell isn't always safe, because any refernece returned from these methods must follow the usual rules.
// Rule - you can't borrow immutably if you have already borrowed mutably!

fn ref_cell_two() {
    let greeting = RefCell::new("world".to_string());
    {
        let mut gr = greeting.borrow_mut();
        *gr = "hello world".to_string();
    }
    println!("value of greeting {:?}", *greeting.borrow());
    assert_eq!(*greeting.borrow(), "hello world");
}


pub fn main() {
    cell_one();
    ref_cell_one();
    ref_cell_two();
}