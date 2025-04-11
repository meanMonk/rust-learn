# Smart Pointers

In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

## Further Information

- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)



# Pointers Vs Smart Pointers.

Pointers
---
 - Pointers are variable which refers to address in memory and this address refers some other data 
 - Reference are indicated by & symbol and borrow the value they point to.

Smart Pointers
---
 - Smart pointers are data structure that act like pointer but also have additional meta data and capabilities.
 - Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that provided by references
 - This pointers enables you to allow the data have multiple owners by keeping track of numbers of owners and when no owners remain, cleaning up data.
 - Reference only borrow data while smart pointers own the data they point to.


### We’ll cover the most common smart pointers in the standard library:

- Box<T> for allocating values on the heap
- Rc<T>, a reference counting type that enables multiple ownership
- Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

- RefCell | Rc
- Mutex | Arc