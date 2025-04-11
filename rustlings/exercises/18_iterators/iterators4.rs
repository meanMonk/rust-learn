fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    
    // if num == 0 {
    //     1
    // } else {
    //     num * factorial(num-1)
    // }
    
    match num {
        0 | 1 => 1, 
        n => n * factorial(n-1)
    }
}

// using product method on iterators.
fn factorial_by_product(num: u64) -> u64 {
    (1..=num).product()
}

// using the fold method. 
fn factorial_by_fold(num: u64) -> u64 {
    (1..=num).fold(1, |acc, x| acc * x)
}

fn factorial_by_for(num: u64) -> u64 {
    let mut result = 1;
    for i in 1..=num {
        result *= i;
    }
    result
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial_by_product(0), 1);
        assert_eq!(factorial_by_fold(0), 1);
        assert_eq!(factorial_by_for(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial_by_product(1), 1);
        assert_eq!(factorial_by_fold(1), 1);
        assert_eq!(factorial_by_for(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial_by_product(2), 2);
        assert_eq!(factorial_by_fold(2), 2);
        assert_eq!(factorial_by_for(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial_by_product(4), 24);
        assert_eq!(factorial_by_fold(4), 24);
        assert_eq!(factorial_by_for(4), 24);
    }
}
