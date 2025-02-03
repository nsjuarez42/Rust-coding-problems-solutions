fn get_digits(n:u64) -> Vec<u8>{
    let mut x : u64 = n;
    let mut digits_arr : Vec<u8> = Vec::new();

    while x>=1{
        digits_arr.push(u8::try_from(x%10).unwrap());
        x /= 10;
    }
    return digits_arr;
}

fn is_balanced(digits:Vec<u8>) -> String{
    if digits.len() <3{
        return "Balanced".to_string();
    }

    if digits.len() %2 == 0{
        if digits[0..digits.len()/2-1].iter().sum::<u8>() == digits[digits.len()/2+1..].iter().sum::<u8>()
        {return "Balanced".to_string()} 
        else 
        {return "Not Balanced".to_string()};

     }else{
        if digits[0..digits.len()/2].iter().sum::<u8>() == digits[digits.len()/2+1..].iter().sum::<u8>() 
        {return "Balanced".to_string()} 
        else {return "Not Balanced".to_string()};
    }
}
pub fn balanced_num(n: u64) -> String{
    let digits : Vec<u8> = get_digits(n);
    println!("{:?}",digits);
    
    return is_balanced(digits);
}

fn factorial(n:u64) -> u64{
    println!("{} n",n);
    let mut z = n;
    let mut fact = 1;
    println!("{} z",z);
    println!("{} fact",fact);
    while z > 1{
        fact = z * fact;
        println!("{}",fact);
        z-=1;
    }
    println!("{}",fact);
    return fact;
}

fn fact(n:u64) -> u64{
    if n < 2{
        return 1;
    }
    return n * fact(n-1);
}

pub fn strong(n:u64) -> String{
    let strong_str = String::from("STRONG!!!!");
    let not_strong_str = String::from("Not Strong !!");
    let mut facts : Vec<u64> = Vec::new();

    for ch in n.to_string().chars(){
        facts.push(fact(ch.to_digit(10).unwrap() as u64));
    }
    if facts.iter().sum::<u64>() == n {return strong_str} else{return not_strong_str};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_number() {
        assert_eq!(balanced_num(7), "Balanced");
        assert_eq!(balanced_num(959), "Balanced");
        assert_eq!(balanced_num(13), "Balanced");
        assert_eq!(balanced_num(432), "Not Balanced");
        assert_eq!(balanced_num(424), "Balanced");
    }
    
    #[test]
    fn larger_number() {
        assert_eq!(balanced_num(1024), "Not Balanced");
        assert_eq!(balanced_num(66545), "Not Balanced");
        assert_eq!(balanced_num(295591), "Not Balanced");
        assert_eq!(balanced_num(1230987), "Not Balanced");
        assert_eq!(balanced_num(56239814), "Balanced");                
    } 
    
    #[test]
    fn strong_nums() {
        // Testing for 1
        assert_eq!(strong(1), "STRONG!!!!");
        
        // Testing for 2
        assert_eq!(strong(2), "STRONG!!!!");
        
        // Testing for 145
        assert_eq!(strong(145), "STRONG!!!!");
        
        // Testing for 7
        assert_eq!(strong(7), "Not Strong !!");
        
        // Testing for 93
        assert_eq!(strong(93), "Not Strong !!");
        
        // Testing for 185
        assert_eq!(strong(185), "Not Strong !!");
    }
}