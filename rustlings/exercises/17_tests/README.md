# Tests

Going out of order from the book to cover tests -- many of the following exercises will ask you to make tests pass!

## Further information

- [Writing Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)


# Bodies of test function perform below actions.
- Set up any needed data or state. 
- Run the code you want to test
- Assert the result what are you expect.
  
  
# Rust provide features for writing test are like 
 -  test attribute 
 -  a few macros
 -  should_panic attribute.
 
To change the function into the test function add `#[test]` line before the `fn` 
Test in rust is function thats annotated with [test] attribute, Attributes are meta data for piece of rust code.
