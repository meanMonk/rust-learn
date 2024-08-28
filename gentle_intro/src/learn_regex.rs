// When processing anything except simple text, 
// Regular expression make your life much easier.


extern crate regex;
use regex::Regex;

pub fn validate_regex() {
    let re = Regex::new(r"(\d{2}):(\d+)").unwrap();
    
    println!("{:?}", re.captures("   10:230"));
    println!("{:?}", re.captures("[22:2]"));
    println!("{:?}", re.captures("10:x23"));
}