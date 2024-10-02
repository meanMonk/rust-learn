pub fn factors(n: u64) -> Vec<u64> {
    // prepare vec of u64 to push values of prime factors.
    let mut prime_factors = Vec::<u64>::new();

    let mut division = n;
    
    let mut divisor = 2;
    
    while division > 1 {
        if division % divisor  == 0 {
            prime_factors.push(divisor);
            division /= divisor;
        } else {
            divisor +=1;
        }
    }
    
    prime_factors
}


// solution 1
/* 

    // prepare vec of u64 to push values of prime factors.
    let mut prime_factors = Vec::<u64>::new();

    let mut division = n;

    for divisor in 2..=n {
        while division % divisor == 0 {
            prime_factors.push(divisor);
            division /= divisor;
        } 
        
        if division == 1 {
            break;
        }
    }
    
    prime_factors

    
*/