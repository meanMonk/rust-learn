/// # Arrays & Slices.
/// array are values packed nose to tail in memory.
/// Arrays are indexed from zero.
/// Arrays are fixed in size,
/// They can be mutable but you can not add new elements to it.

fn array_one() {
    let arr = [10, 20, 30, 40, 50];
    // As arrays are not used that often in Rust, because the type of an array includes its size
    let ele_first = arr[0];
    println!("first {ele_first}");

    // if increase index return out of bound.
    for i in 0..5 {
        println!("[{i}] {}", arr[i]);
    }
    println!("length {}", arr.len());
}

// focus on &[i32] Rust slices keep track of their size (and will panic if you try to access outside that size)
// and you have to explicitly say that you want to pass an array as a slice using the & operator.

fn sum_of_array(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

// read as slice of i32
fn array_two() {
    let arr = [10, 20, 30, 40];
    let sum = sum_of_array(&arr);
    println!("Sum {} Array elements {:?}", sum, arr);
}

// `{}` won't work to print out the arr we need to use {:?} - debug print.
fn array_slicing_dicing() {
    let ints = [1, 2, 3, 5];
    let floats = [1.3, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [4, 5]]; // arrays of array no problem but imp that an array contains values of only one type.

    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);

    // slices give different view of same array.
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];
    println!("slice1 {:?}", slice1);
    println!("slice1 {:?}", slice2);

    // Note: A copy of the data is never made.
    // These slices all borrow their data from their arrays. They have a very intimate relationship with that array
}

/*
    Slices, like arrays, can be indexed.
    Rust knows the size of an array at compile-time,
    but the size of a slice is only known at run-time.
    So s[i] can cause an out-of-bounds error when running and will panic and there are no exception on compile time.
    ` can not wrap the code and catch the errors` so how rust is safe?
*/

// Option : You can think of Option as box which may contain a value or nothing (None)
// It is very common for Rust functions/methods to return such maybe-boxes

fn slice_one() {
    let ints = [1, 2, 3, 4, 5, 6];
    let slice = &ints;
    // SLICE HAS METHOD  `get` which doesn't panic
    // as length of slice is known at run time so can caouse out of bounds error.
    let first_of_slice = slice.get(0); // output `first Some(1)`
    let last_of_slice = slice.get(6); // output `last None`

    println!("first {:?}", first_of_slice);
    println!("last {:?}", last_of_slice);
    // Returns Option which has some methods.

    println!(
        "first value {} isNone {} isSome {} ",
        first_of_slice.unwrap(),
        first_of_slice.is_none(),
        first_of_slice.is_some()
    );

    // as for last unwrap  fails we can check first and then perform unwrap.
    let last = if last_of_slice.is_some() {
        *last_of_slice.unwrap() // option is of &i32 so we need to use dereference `*` to get back value i32
    } else {
        -1
    };

    let short_last = *slice.get(6).unwrap_or(&-1);

    println!(
        "last values {last} shortLast {short_last} isNone {} isSome {}",
        last_of_slice.is_none(),
        last_of_slice.is_some()
    );
}

pub fn array_slices_main() {
    array_one();
    array_two();
    array_slicing_dicing();

    slice_one();
}
