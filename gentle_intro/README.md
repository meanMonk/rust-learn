# Rust

> Learning a language means getting comfortable with its errors.

- Rust is curly braces language with semicolons
- The exclamation mark indicates that this is macro call.
- Spelling mistakes are compile errors, not runtime errors like dynamic lang python or js.
- macros  
  - `println!`
  - `assert_eq!` use for assertion to validate the values don't produce any result but panic on fails 
- Rust is both **statically-typed** and **strongly-typed** langauge - can be often confused.
  - In static types the type is known at compile time, and dynamic types are only known at run time.
- `traits` (+=) is trait named as `AddAssign` full [list of trais ](https://doc.rust-lang.org/std/ops/index.html)
- The `()` type is empty type, nada, void, zilch, nothing. everything in rust has value but sometime it's nothing.
- It's not wrong to use `return`, but code is cleaner without it. you will still use `return` for returning early from function.
- Values can also be passed by reference. A reference is created by `&` and dereferenced by `*`.