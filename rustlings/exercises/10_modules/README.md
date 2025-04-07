# Modules

In this section we'll give you an introduction to Rust's module system.

## Further information

- [The Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)


## crate: 
- Crates are smallet amount of code that Rust compilers considers at time.
- Crates can contain modules and module may be defined in other files that gates compiled with other crate.

  1. Binary Crate  // can have main funtion
  2. Library Crate  // can not have main function

## Package:
- Packages is set of one or more crates that provides set of functionalities.
- A package contain cargo.toml which describes how to build those crates.

## Module:
- Module let us organize the code in crate for readability and reuse.
- Allow us to control the privacy as code in module default private.
- 