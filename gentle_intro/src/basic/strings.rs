// Strings.
// `String` type like `Vec` allocates dynamically and resizable.
// In embedded micros, that could mean putting them in cheap ROM rather than expensive RAM
// (for low-power devices, RAM is also expensive in terms of power consumption.)
// A system language has to have two kinds of string,
//      1. allocated
//      2. static.
// You can convert to slice with same way like array
// But you can not indexed string.
/*
    "hello" is not of type `String`. It is of type `&str`
    `&str` > pronounced as `string slice`
    // it's more intelligent
    `&str` and `String` have more similar relation to each other
*/

// Under the hood string is basically a Vec<u8>  and &str is &[u8]

// The `format!` is very useful macro to build up more complicated strings using the same format string as println!.

fn string_methods() {
    let multilingual = " Hi! ¡Hola! привет! namste!";
    for ch in multilingual.chars() {
        print!("{:?}", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("chars len {}", multilingual.chars().count());

    // check for namste and print onkly.
    // find for ch finding
    // get for getting value of index.
    let maybe = multilingual.find('n');

    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("In hindi hi {}", hi);
    }

    // collect
    // split_whitespaces.

    let words: Vec<&str> = multilingual.split_whitespace().collect();
    println!("words vlaue {:?}", words);

    // can be write in way.

    let stripped: String = multilingual
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect();

    println!("stripped string {:?}", stripped);
}

fn string_slice_like_notation() {
    // notation used for slice works with strings as well.
    // Note: but can index string.
    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..]; // creating slice of string
    let string_s = &string[2..4]; // creating slice of string.

    println!("slices {:?} - {:?}", text_s, string_s)
}

fn array_to_string(arr: &[i32]) -> String {
    let mut s = "[".to_string();

    for num in arr {
        s += &num.to_string(); // operation defines on slice of `&str` not on String.

        s.push(';');
    }

    s.pop();
    s.push(']');
    s
}

fn string_push_pop() {
    let mut s = String::new();

    s.push('H');
    s.push_str("ello ");
    s += "World!";
    s.push('!');

    println!("string after push {:?}", s);

    s.pop();
    println!("string after pop {:?}", s);
}

fn dump(s: &str) {
    println!("str = {:?}", s);
}

fn string_one() {
    let text = "Hello Dolly"; // the string slice;
    let s = text.to_string(); // it's no an allocated string.

    dump(text);
    dump(&s); // & operator coerce `String` into `&str`
}

pub fn string_main() {
    println!("||| STRING |||");
    string_one();
    string_push_pop();

    let arr = [11, 22, 33, 44, 55];
    let stringified_arr = array_to_string(&arr);
    let res = format!("Hello {}", stringified_arr);
    println!("Formated string {res}");

    string_slice_like_notation();

    string_methods();
}
