use regex::Regex;
use time::OffsetDateTime;
fn main() {
    println!("Hello, world!");
    
    let name = String::from("Rustacian Bro!");
    println!("Good Morning {}", name);
    
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    
    let now = OffsetDateTime::now_utc();
    
    println!("Did our {now} match? {}", re.is_match(&now.to_string()));
    
}
