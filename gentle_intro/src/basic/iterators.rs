// ITERATORS
// An iterator is easy to define informally.
// It is an 'object' with `next` method and returns an `Option`.
// As long as value is not `None` we keep calling `next`;
// And that is exactly what `for var in iter {}` does.

fn iterator_one() {
    let mut iter = 0..3;

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));

    let mut iter = 0..3;
    println!("index i {:?}", iter.next());
    println!("index i {:?}", iter.next());
    println!("index i {:?}", iter.next());
}

// attempt to iterae orver an array.
/*
    fails with iterator is not implemented for array.

    fn main() {
        let arr = [10,20,30];
        for i in arr {
            println!("{}",i);
        }
    }

*/

// better than iterating on range like 0..slice.len()
fn iterator_second() {
    let arr = [10, 20, 30];

    for i in arr.iter() {
        println!("{}", i);
    }

    // NOTE: slices will be converted implicitly to iterators...

    for i in &arr {
        println!("slice iterator {i}");
    }
}

// pro level of doing sum is

fn pro_sum() {
    let sum: i32 = (0..3).sum();
    println!("sum with ranges {sum}");

    let sum: i32 = [2, 3, 4].iter().sum();
    println!("sum with ranges {sum}");
}

// Windows method gives you an iterator of slices.

fn window_slice() {
    let ints = [1, 2, 3, 4, 5, 6];

    let slice = &ints;

    for s in slice.windows(2) {
        println!("window is {:?}", s);
    }

    for s in slice.windows(3) {
        println!("window of 3 is {:?}", s);
    }
}

fn chunks_slice() {
    let ints = [11, 22, 33, 44, 55, 66, 77];

    let slice = &ints;

    for s in slice.chunks(2) {
        println!("Chunks of 2 {:?}", s);
    }
}

pub fn iterators_main() {
    iterator_one();

    iterator_second();

    pro_sum();

    window_slice();

    chunks_slice();
}
