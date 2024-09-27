// Multithreading
// To get most out of modern computer is to keep all of those cores busy.
// it's possible to spawn child process in the background as we saw with `Command`
// but there's synchronization problem.
// we don't know when those children are finished without waiting on them.

// there are other reason for needing separate threads of execution of course.
// you can not afford to lockup your whole process just to wait on blocking I/O for instance.

// spwaning thread is straightforward in `Rust`

// There is no particular order to thread execution (this program gives different orders for different runs), 
// and this is key - they really are independent threads of execution. 
// Multithreading is easy; what's hard is concurrency - managing and synchronizing multiple threads of execution.



use std::{thread::{self}, time};

fn thread_one() {
    thread::spawn(|| println!("hello!"));
    thread::spawn(|| println!("dolly!"));
    
    println!("so fine");
    
    thread::sleep(time::Duration::from_millis(100));
}

// it's better to call `join` on the returned object - then the main thread waits for the spawned thread to finish.

fn thread_two() {
    let t = thread::spawn(|| {
        println!("hello from child thread");
    });
    
    println!("wait for {:?}",t.join());
}

// force the new thread to panic.

// we get a panic as expected and but only the panicking thread dies!
// we still manage to print out the error message from `join`.
// so yes panics are not always fatal, but threads are relatively expensive, so this should not be seen as routie way of handling panics.

fn thread_three() {
    let t = thread::spawn(|| {
        println!("hello goes!");
        panic!("I gave up to erro!");
    });
    
    println!("wait  {:?}", t.join());
}


// return objects can be used to keep track of multiple threads;

fn thread_four() {
    let mut threads = Vec::new();
    
    for i in 0..5 {
        let t = thread::spawn(move || {
            println!("hello {}", i);
        });
        threads.push(t);
    }
    
    for t in threads {
        t.join().expect("thread failed");
    }
}

// #Threads don't borrows.
// It's possible for thread closure to capture the value but by moving, not by borrowing!
// spawing this thread from function - it will exist after the function call has finished and name gets dropped.
// adding `move` solves our problem.
fn thread_five() {
    let name = "hello".to_string();
    
    let t = thread::spawn(move || {
        println!("world says {}", name);
    });
    
    println!("wait {:?}", t.join());
}

// by adding `move` so `name` only appear in one thread! I'd like to emphasize that it is possible to share ref.

fn thread_six() {
    let name ="dolly";
    let t1 = thread::spawn(move || {
        println!("t1 says {}", name);
    });
    let t2 = thread::spawn(move || {
        println!("t2 says {}", name);
    });
}

// You can not use Rc as it's not thread safe - it's optimized to be fast for the non-threaded case.
// it's compile error to use `Rc` in threads.

// For threads you need to use `std::async:Arc` `Arc` - `Automatic Reference Counting`
// that is, it guarantees that ref count will be modified in one logical operation. 
// To make this guarantee, it must be ensure that the operation is locked so that only the current thread has access.
// `clone` is much cheaper than actually making copy however.

use std::sync::Arc;

struct MyString(String);

impl MyString {
    fn new(s: &str) -> MyString {
        MyString(s.to_string())
    }
}

fn thread_seven() {
    let mut threads = Vec::new();
    let name = Arc::new(MyString::new("dolly"));

    for i in 0..5 {
        let tname = name.clone();
        let t = thread::spawn(move || {
            println!("hello {} count {}", tname.0, i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}

// The shared reference name is passed to each new thread by making a new reference with clone and moving it into the closure. 
// It's a little verbose, but this is a safe pattern.

pub fn main() {
    thread_one();
    thread_two();
    thread_three();
    thread_four();
    thread_five();
    thread_six();
    thread_seven();
}