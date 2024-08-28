use regex::Regex;
use time::OffsetDateTime;

mod file_sys_programs;
mod process_programs;

mod foo;
mod boo;

/* 
    Please note that 
    `use` 
    has nothing to do with importing, 
    It simply specifies visibility of module names. For example:
    
    {
        use boo::bar;
        let q = bar::question();
    }
    {
        use boo::bar::question;
        let q = question();
    }

*/

fn main() {
    println!("Hello, world!");
    
    let name = String::from("Rustacian Bro!");
    println!("Good Morning {}", name);
    
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    
    let now = OffsetDateTime::now_utc();
    
    println!("Did our {now} match? {}", re.is_match(&now.to_string()));
    
    file_sys_programs::read_all_lines("sample.txt").expect("read failed");
    
    file_sys_programs::write_out("test.txt").expect("write failed");
    
    file_sys_programs::print_cargo_path().expect("print home failed");
    
    file_sys_programs::travel_to_home_dir().expect("failed to travel back!");
    
    file_sys_programs::print_readme_path().expect("failed to load readme path");
    
    file_sys_programs::file_meta_data();
    
    // process related programs.
    println!("💻💻💻💻");
    process_programs::check_rustc();
    process_programs::check_rustc_output();
    
    let f = foo::Foo::new("hello struct");
    println!("👍{:?}", f);
    
    println!("Answer is {}", boo::answer());
    
    println!("{}❓", boo::bar::question());

}
