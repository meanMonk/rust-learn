// Structs
// better than tuple as we don't need to keep track of meaning of each element of like Tuples.
// Structs contain named fields.

// Summarize:
//  - no `self` arguments: you can associate functions with structs, like the new "constructor"
//  - `&self` argument: can use value of struct but not change theme.
//  - `&mut self` argument: can modify the values of struct.
//  - `self` argument: will consume the value and which will move.

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // not reserved words so can be anything.
    fn new(first_name: &str, last_name: &str) -> Self {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    // here's method that takes &self argument.
    // &self is short of &Person.
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // the keyword `Self` refer to struct type.

    fn copy(&self) -> Self {
        Person::new(&self.first_name, &self.last_name)
    }

    // methods may allow data to be modified using the mutable self arguments.

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    // data will move into the method when plain self is used.

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

fn struct_learn() {
    let person = Person {
        first_name: "Hello".to_string(),
        last_name: "John".to_string(),
    };

    // to print it need to make debug drive.
    println!("person struct value {:#?}", person);
    println!("Name {} {}", person.first_name, person.last_name);

    // initilizing struct is bit clumsy so we want to move construction of `Person` to it's own function
    // with help of impl

    let mut c_person = Person::new("Milli", "Watson");

    println!("Constructed struct {:#?}", c_person);
    println!("Name {} {}", c_person.first_name, c_person.last_name);

    println!("Full Name : {}", c_person.full_name());

    let c_person_one = c_person.copy();
    println!("Copied Full Name : {}", c_person_one.full_name());

    c_person.set_first_name("Bill");

    println!("updated full name {}", c_person.full_name());

    let name_tuple = c_person.to_tuple();

    // println!("No data person {}", c_person.full_name()); // panic as ownership of data moved to tuple.
    println!("No data person {:?}", name_tuple);
}

// LifeTimes Start to Bite.
// usually `Struct` contains the values but ofthen they also need to contain references. say we want to put a string slice, not string value in struct.

/*
    Error - missing lifetime specifier
     - Rust will not allow a reference to be stored without knowing it's lifetime. all reference borrowed from some value.
     - Life time of references can not be longer than the lifetime of value.
     - string slices borrow from string literals like "hello" or from `String` values. exist for duration of the whole program which is called
     `static` lifetime.

     `&'static str` - static lifetime as it's reference to String literals which is valid for duration of whole program.
*/

#[derive(Debug)]
struct A {
    name: &'static str,
}

// &'static can be used specify string slice that is returned from fn.
// That works for the special case of static strings, but this is very restrictive.

fn how(i: u32) -> &'static str {
    match i {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        _ => "Many",
    }
}

// we can specify that the lifetime of the reference is at least as long as that of struct itself.
// lifetime conventionally called 'a', 'b' or you could just call it 'me' as well.
#[derive(Debug)]
struct B<'a> {
    name: &'a str,
}

// there is no way this could work as string is dropped when the function ends.
// no reference to string can outlast it.
/*
   fn create_b () -> B<'static> {
       let s = "I'm a little string".to_string();
       B { name: &s }
   }
*/

fn struct_life_learn() {
    let a = A { name: "hello dam" };
    println!("value of a {:?}", a);

    println!("0 is {}", how(0));
    println!("1 is {}", how(1));
    println!("2 is {}", how(2));
    println!("4 is {}", how(4));

    let s = "I'm a little string".to_string();
    let b = B { name: &s };
    println!("{:?}", b);
    // println!("{:?}", create_b());
}

pub fn main() {
    struct_learn();
    struct_life_learn();
}
