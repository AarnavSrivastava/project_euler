fn main() {
    println!("{}", champernownes_constant(vec![1, 10, 100, 1_000, 10_000, 100_000, 1_000_000]));
}

fn champernownes_constant(req_digits: Vec<usize>) -> i32 {
    let mut digits = vec![0];
    let n = req_digits.len() - 1;
    let max_digit = req_digits[n];

    for i in 0..=max_digit {
        let mut temp = i;
        let pointer = digits.len();

        while temp > 0 {
            digits.insert(pointer, temp % 10);
            temp /= 10;
        }
        
        if digits.len() > max_digit {
            break;
        }
    }

    let mut prod = 1;
    for i in req_digits {
        prod *= digits[i];
    }
    
    prod as i32
}