#[derive(Debug)]
struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut counter = Counter::new();
    println!("{}", counter.next().unwrap());
    assert_eq!(counter.next(), Some(2));

    let mut values = vec![41];
    for x in values.iter_mut() {
        *x += 1;
    }
    for x in values.iter() {
        assert_eq!(*x, 42);
    }
    assert_eq!(values.len(), 1); // `values` is still owned by this function.
    
    for x in &mut values { // values.iter_mut()
        *x += 2;
    }
    
    for x in &values { // same values.iter()
        assert_eq!(*x, 44)
    }
    
    assert_eq!(values.len(),1)
}
