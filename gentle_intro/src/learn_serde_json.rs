// serde_json
// use to serialize Rust Data structure to JSON.
// use to deserialize Json to Rust Data Strcture.

// serde seralize / deserialize also support other file format.

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}
//
#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

pub mod learn_serde {
    use crate::learn_serde_json::Person;

    pub fn use_serde() {
        let data = r#"{
            "name" : "sahil",
            "age":30,
            "address": {
                "street": "road 23",
                "city": "pune"
            },
            "phones": ["61238923"]
        }"#;

        let p: Person = serde_json::from_str(data).expect("deserialize error");
        println!("please call {} , at the number {}", p.name, p.phones[0]);

        println!("{:#?}", p);
    }
}
