// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers: Vec<i16> = Vec::new();

    /* 
        NOTE:
        Vec<i16>: This is a vector of i16 integers. The i16 type is a 16-bit signed integer, which can accommodate both u8 (0 to 255) and i8 (-128 to 127) values.
        By choosing i16, you ensure that the vector can store values from both u8 and i8 without issues.
    
     */
    // i16 can accomodate `u8` and `i8`
    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
