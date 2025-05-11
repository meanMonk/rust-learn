// implement the smart pointer Box<T> accesing the data.
#[allow(dead_code)]

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::sync::{Arc,Mutex};
use std::thread;

use crate::List::{Nil, Cons};

fn main() {
    let b = Box::new(5);
    println!("Hello, world!");
    println!("Boxes values {b}");
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    println!("{:?}", list);
    
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y);
    
    // mutex : mutual exclusion
    
    let m = Mutex::new(5);
     {
        let mut num = m.lock().unwrap();
        *num = 6;
     } 
     println!("m = {m:?}");
     
     
    //  Mutex<T> - mutex with thread 
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    
}
