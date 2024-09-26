/* 
    There is a downside to using this crate, because of the mismatch between the amorphous, 
    dynamically-typed nature of JSON and the structured, static nature of Rust. 
    (The readme explicitly speaks of 'friction') So 
    if you did want to map JSON to Rust data structures, 
    you would end up doing a lot of checking, 
    because you can not assume that the received structure matches your structs! 
    For that, a better solution is serde_json where you serialize 
    Rust data structures into JSON and deserialize JSON into Rust.

*/

// #[macro_use]
// extern crate serde_derive;
extern  crate serde_json;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String, 
    age: u8, 
    address: Address, 
    phones: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String, 
    city: String
}


fn serialize_one() {
    let data = r#"{
        "name" : "John Does",
        "age": 42,
        "address": {
            "street": "punawalet",
            "city":"pune"
        },
        "phones": ["8843348848"]
    }"#;
    
    let p: Person = serde_json::from_str(data).expect("deserialize error!");
    
    println!("please call {} at number {}", p.name, p.phones[0]);
    
    println!("Person {:?}", p)
}


pub fn main() {
    println!("Learn Serde Json | Serialize <> Deserialize");
    
    serialize_one();
}