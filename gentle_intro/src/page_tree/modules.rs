// Modules.
// -- In a perfect world a module does one thing, does it well, and keeps its own secrets.

// As program gets larger it's necessory to spread them over the more than one file
// and put functions and types in different namespaces, the Rust solution for both of these is modules.

// In Rust rust the full name would like
// `primitive::display::set_width` after saying `use primitive::display` you can refer it as `display::set_width`
// also we can do `use primitive::display::set_width` and use it as set_width but not good practice as this can leads to confusion later.

// `mod` is used to define a module as block where Rust types or functions can be written:

/**
   mod foo {
        #[derive(Debug)]
        struct Foo {
            s: &'static str
        }
   }

   fn main() {
        let f =  foo::Foo{s: "hello"};
        println!("{:?}",f);
   }
*/

/*

    In summary,
        - modules are about organization and visibility,
        - and this may or may not involve separate files.

    NOTE:
        - Please note that use has nothing to do with importing,
        and simply specifies visibility of module names. For example:

    IMPORTANTPOINT 💡:
        - There is no separate compilation here.
        - main program and it's module files will be recompiled each time.

*/

mod foo {

    // need to add `pub` keyword to export `Foo`
    #[derive(Debug)]
    pub struct Foo {
        pub s: &'static str, // strings slices borrowed from string literals exit for the duration of whole program. which called 'static'
    }

    impl Foo {
        // function to create new struct.
        // Why hiding the implementation a good thing?
        //      📓 as we can change later without breaking interface.
        //      📓 The great enemy of large-scale programing is a tendency for code to get entangled,
        //      📓 so that understanding a piece of code is impossible in isolation.

        pub fn new(s: &'static str) -> Foo {
            Foo { s: s }
        }
    }
}

// In perfect world module does one thing, does it well, and keep its own secrets.

// FOO code to "./foo.rs"
// use it "mod foo;"
// compiler also looks for "<MODNAME>/mod.rs" so we can say "foo/mod.rs" with foo implementation

pub fn main() {
    let f = foo::Foo::new("Hello");
    println!("foo struct > {:?}", f);
}
