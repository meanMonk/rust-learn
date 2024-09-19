# Rust

> Learning a language means getting comfortable with its errors.

- Rust is curly braces language with semicolons
- The exclamation mark indicates that this is macro call.
- Spelling mistakes are compile errors, not runtime errors like dynamic lang python or js.
- macros  
  - `println!`
  - `assert_eq!` use for assertion to validate the values don't produce any result but panic on fails 
  - `assert! , assert_eq!` this is the workhorse of testing in rustl
- Rust is both **statically-typed** and **strongly-typed** langauge - can be often confused.
  - In static types the type is known at compile time, and dynamic types are only known at run time.
- `traits` (+=) is trait named as `AddAssign` full [list of trais ](https://doc.rust-lang.org/std/ops/index.html)
- Rust likes to be explicity- it will not silently convert that integer inot a float for you we need to cast that to floating point values explicitly.
- The `()` type is empty type, nada, void, zilch, nothing. everything in rust has value but sometime it's nothing.
- It's not wrong to use `return`, but code is cleaner without it. you will still use `return` for returning early from function.
- Values can also be passed by reference. A reference is created by `&` and dereferenced by `*`.
- Where to find repo.
  - `rustup doc --std`
  - `rustup doc --help`
- `std::f64::consts::PI` is a mouthful! `::` means much the same as it does in C++, (often written using `'.'` in other languages) - it is a fully qualified name. We get this full name from the second hit on searching for PI.
- `&` rust programmer pronouces it as `borrow`