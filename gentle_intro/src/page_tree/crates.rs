// Crates
// - `compilation unit` for Rust is the crate, which is either an executable or a library.
// To separately compile the files from the last section, first build foo.rs as rust static library and crate:

// rustc foo.rs --crate-type=lib
// this creates `libfoo.rlib`

// -> to link this into our main programs.
// `rustc mod4.rs --extern foo=./libfoo.rlib`

// so main.rs looks like
/* 
     extern crate foo;
     
     fn main() {
        foo::Foo::bar::question();
     }
     
 */
// see the rust binaries size.

// `ls -lh main`
// to reduce Bad thing
// `strip main`


// This is a Good Thing, 
// since you can hand this executable to anyone with the right operating system 
// - they will not need a 'Rust runtime'. 
// (And rustup will even let you cross-compile for other operating systems and platforms as well.)

// we can access community provided library with help crates.io using Cargo.
// cargo will look up correct version and download source for us.


// Cargo
// Rust std library is not very large compared to java and python.

// it's straightforward to access community provided libraries in `crates.io` using cargo.
// cargo check for version & download source for you.
// ensure to download deps crates as well.

// to add new deps edit `cargo.toml`
// to do the build with cargo
// `cargo build`

// to build and run the programs.
// `cargo run`

// 