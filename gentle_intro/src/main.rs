mod basic;
mod greet;
mod page_tree;
mod page_two;
mod page_five;
mod page_six;
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
    greet::greet("Rust Gentle Intro");

    basic::main();
    page_two::main();
    page_tree::main();
    page_five::main();
    page_six::main();
}
