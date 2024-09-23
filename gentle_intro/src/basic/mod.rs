
use crate::greet;

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
mod looping_ifying;


pub fn main() {
    greet::greet("Learning Basics");
    
    
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
    
    // need to supply file name here!
    // reading_files::main();
    
}