use std::fmt::Debug;

use crate::greet;
// Generic Struct
// Considering earlier ./struct_dynamic_data.rs (binary tree)
// it will be irritating to have to rewrite for all possible kinds of payload as that only support `String` type.
// Let's implement generic Node with it's type parameter `T`


type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    payload: T, 
    left: NodeBox<T>,
    right: NodeBox<T>,
}

// fundamental operation of payload is comparison so T must be comparable with `<` 
// i.e. implementation of `PartialOrd`. The type parameter must be declared in the `impl` block with it's constraints.

impl <T: PartialOrd + Debug> Node<T> {
    fn new(s:T) -> Node<T> {
        Node {
            payload: s, 
            left: None,
            right: None
            
        }
    }
    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }
    
    fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node);
    }
    
    fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node);
    }
    
    fn insert(&mut self, data:T) {
        if data < self.payload {
            match  self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data))
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }
    
    fn visit(&self) {
        if let Some(ref left ) = self.left {
            left.visit()
        }
        
        println!("'{:?}'", self.payload);
        if let Some(ref right ) = self.right {
            right.visit()
        }
        
    }
    
}


fn generic_binary_tree_one() {
    let mut root = Node::new("Root".to_string());
    root.insert("one".to_string());
    root.insert("three".to_string());
    root.insert("zero".to_string());
    root.insert("five".to_string());
    
    println!("root {:#?}", root);
}

fn generic_binary_tree_two() {
    let mut root = Node::new(20);
    root.insert(11);
    root.insert(22);
    root.insert(12);
    root.insert(09);
    
    println!("root {:#?}", root);
    println!("Traversing through Tree!");
    root.visit();
}


pub fn main() {
    greet::greet("Generic Struct");
    
    generic_binary_tree_one();
    generic_binary_tree_two();
}