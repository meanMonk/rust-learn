// Channels
// There are way to send data between threads
// This is done in Rust using channels.
// `std::sync::mpsc::channel()` return a tuple consisting of receiver channel and sender channel
// 

use std::{sync::mpsc, thread};

fn channel_one() {
    let nthreads = 5;
    
    let (tx, rx) = mpsc::channel();
    
    for i in 0..nthreads  {
        let tx = tx.clone();
        thread::spawn(move || {
            let response = format!("hello from {}", i);
            tx.send(response).unwrap();
        });
    }
    
    for i in 0..nthreads {
        println!("go {:?}", rx.recv());
    }
}

// theres no need to join here since threads send their response just before they end execution, but obviously this can happen at. 
// any time. `recv` will block and will return an error if the sender channel is diconnected.
// `send` never blocks which is useful becuase threads can push out data without waiting for the receiver to process.
// in addition, the channel is buffered so multiple send operations can take place, which will be received in order.

// However, not blocking means that Ok does not automatically mean 'successfully delivered message'!
// `sync_channel` does block on send.

// if the `sync_channel` was created with a non-zero argument 'n', then it's act like queue with a mazimum size of `n` 
// - `send` will only block when it tries to add more than `n` values to the queue

fn channel_two() {
    let (tx, rx) = mpsc::sync_channel(0);

    let t1 = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });

    for _ in 0..5 {
        let res = rx.recv().unwrap();
        println!("sync got {}",res);
    }
    t1.join().unwrap();
}

pub fn main() {
    channel_one();    
    channel_two();
}
