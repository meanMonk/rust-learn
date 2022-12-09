//  tuples group together values of different types. like object/array in JS.
// Max 12 elements.

pub fn run() {
  let person :(&str, &str, &char ) = ("Hello", "World", &'\u{1F600}');
  
  println!("{} is from {} with {} ", person.0, person.1, person.2 );
}