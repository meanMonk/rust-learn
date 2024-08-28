use regex::Regex;
use time::OffsetDateTime;

mod file_sys_proces;

fn main() {
    println!("Hello, world!");
    
    let name = String::from("Rustacian Bro!");
    println!("Good Morning {}", name);
    
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    
    let now = OffsetDateTime::now_utc();
    
    println!("Did our {now} match? {}", re.is_match(&now.to_string()));
    
    file_sys_proces::read_all_lines("sample.txt").expect("read failed");
    
    file_sys_proces::write_out("test.txt").expect("write failed");
    
}
