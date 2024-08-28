use regex::Regex;
use time::OffsetDateTime;

mod file_sys_process;

fn main() {
    println!("Hello, world!");
    
    let name = String::from("Rustacian Bro!");
    println!("Good Morning {}", name);
    
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    
    let now = OffsetDateTime::now_utc();
    
    println!("Did our {now} match? {}", re.is_match(&now.to_string()));
    
    file_sys_process::read_all_lines("sample.txt").expect("read failed");
    
    file_sys_process::write_out("test.txt").expect("write failed");
    
    file_sys_process::print_cargo_path().expect("print home failed");
    
    file_sys_process::travel_to_home_dir().expect("failed to travel back!");
    
    file_sys_process::print_readme_path().expect("failed to load readme path");
    
    file_sys_process::file_meta_data();
}
