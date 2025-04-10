// We don’t need to annotate any code in tests/integration_test.rs 
// with #[cfg(test)]. Cargo treats the tests directory specially and 
// compiles files in this directory only when we run cargo test.
//  Run cargo test now:

use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(3);
    println!("result is like {result}");
    assert_eq!(result, 5);
}