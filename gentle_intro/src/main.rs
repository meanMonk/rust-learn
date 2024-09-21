use regex::Regex;
use time::OffsetDateTime;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod arrays_slices;
mod function_and_more;
mod iterators;
mod option;
mod vectors;
mod strings;
mod interlude;
mod matching;
mod reading_files;


mod file_sys_programs;
mod process_programs;

mod boo;
mod foo;

mod learn_json;
mod learn_serde_json;

mod learn_date_time;
mod learn_regex;

mod looping_ifying;
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
    println!("Hello, world!"); // The exclamation mark(!) indicates that this is macro call.

    looping_ifying::loop_main();

    function_and_more::fn_more_main();

    arrays_slices::array_slices_main();

    option::option_main();

    vectors::vec_main();

    iterators::iterators_main();
    
    strings::string_main();
    
    interlude::interlude_main();

    matching::matching_main();
    
    file_sys_programs::fs_program_main();
    
    reading_files::main();
    // introduce variable.
    // let name: String = String::from("Rustacian Bro!");
    // println!("Good Morning {}", name);

    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    // let now = OffsetDateTime::now_utc();

    // println!("Did our {now} match? {}", re.is_match(&now.to_string()));

    // file_sys_programs::read_all_lines("sample.txt").expect("read failed");

    // file_sys_programs::write_out("test.txt").expect("write failed");

    // file_sys_programs::print_cargo_path().expect("print home failed");

    // file_sys_programs::travel_to_home_dir().expect("failed to travel back!");

    // file_sys_programs::print_readme_path().expect("failed to load readme path");

    // file_sys_programs::file_meta_data();

    // process related programs.
    // println!("💻💻💻💻");
    // process_programs::check_rustc();
    // process_programs::check_rustc_output();

    // let f = foo::Foo::new("hello struct");
    // println!("👍{:?}", f);

    // println!("Answer is {}", boo::answer());

    // println!("{}❓", boo::bar::question());

    // learn_json::print_json();

    // learn_serde_json::learn_serde::use_serde();

    // learn_regex::validate_regex();

    // learn_date_time::review_date();
}
