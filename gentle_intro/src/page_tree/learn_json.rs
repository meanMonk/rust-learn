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
    
    // can do asserstion.
    let code = doc["code"].as_u32().unwrap_or(0);
    let status = doc["success"].as_bool().unwrap_or(false);
    assert_eq!( code,200);
    assert_eq!( status,true);
    
    let features = &doc["payload"]["features"];
    
    for v in features.members() {
        println!("{v}");
    }
    
}
