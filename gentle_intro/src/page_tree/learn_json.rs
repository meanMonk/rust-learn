extern crate json;

pub fn main() {
    println!("Learning Json");

    let doc = json::parse(
        r#"
{
    "code": 200, 
    "success": true, 
    "payload": {
        "features": [ "demo 1", "demo 3", "mac app"]
    }   
}    
    "#,
    )
    .expect("parse failed");

    println!("debug {:?}", doc);
    println!("normal {}", doc);
}
