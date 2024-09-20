use json::{array, object};

extern crate json;

pub fn print_json() {
    let doc = json::parse(
        r#"{
        "code": 200,
        "success":true, 
        "payload": {
            "features":[
                "awesome",
                "rust",
                "programming"
            ]
        }
    }"#,
    )
    .expect("parse failed");

    println!("debug {:?}", doc);
    println!("display {:#}", doc);

    // let's do assertion and check values.

    let code = doc["code"].as_u32().unwrap_or(0);
    let status = doc["success"].as_bool().unwrap_or(false);
    assert_eq!(code, 200);
    assert_eq!(status, true);
    println!("{}", code);
    println!("{}", status);

    let features = &doc["payload"]["features"];

    for v in features.members() {
        println!("F- {}", v.as_str().unwrap());
    }

    // use macros to generate the JSON literals.

    let user = object! {
        "name" => "Rustacian",
        "languages" => array!["Rust", "Go", "Solidity"],
        "canProgram" => true
    };
    let dump = user.dump();
    println!("🦸 : {:#}", json::parse(&dump).expect("parse failed!"));
}

/*
// There is a downside to using this crate, because of the mismatch between the amorphous,
// dynamically-typed nature of JSON and the structured, static nature of Rust.
// (The readme explicitly speaks of 'friction')
// So if you did want to map JSON to Rust data structures,
// you would end up doing a lot of checking,
//  because you can not assume that the received structure matches your structs! For that,
// a better solution is
// `serde_json` where you serialize Rust data structures into JSON and deserialize JSON into Rust.
*/
