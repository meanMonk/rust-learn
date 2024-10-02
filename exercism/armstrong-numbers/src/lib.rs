pub fn is_armstrong_number(num: u32) -> bool {
    let digit_count = num.to_string().len() as u32;
    let mut total: u32 = 0;
    let mut my_num = num;
    while my_num > 0 {
        let digit : u32 = my_num % 10;
        total += digit.pow(digit_count);
        
        if total > num {
            return false;
        }
        
        my_num /= 10;
    }
    
    num == total
}

/* 
// Solution 2

pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();

    let input_vec: Vec<_> = num_str.split("").filter(|s| !s.is_empty()).collect();

    let num_len = input_vec.len() as u32;
    // arm strong number is sum of power of each digit by it's position
    let result = input_vec.iter().fold(0, |acc: u32, ele| {
        let ele: u32 = ele.parse::<u32>().ok().unwrap();
        acc + ele.pow(num_len)
    });

    if result == num {
        return true;
    } else {
        return false;
    }
}
*/

// solution to directly change number to vector by dividing by 10.
/* 

    // solution 1
pub fn is_armstrong_number(num: u32) -> bool {
    let mut input_num = num;
    let mut digits = Vec::new();
    
    while input_num > 0 {
      digits.push(input_num % 10);
      input_num = input_num / 10;
    }
    
    let length = digits.len() as u32;
    
    num == digits.into_iter().fold(0, |acc, ele| acc + ele.pow(length))

}
*/