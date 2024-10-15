fn main() {
    // println!("{}", is_palindrome(121));
    println!("{}", lychrel_numbers(10_000));
}

fn lychrel_numbers(limit: usize) -> i32 {
    let mut sieve = vec![true; limit + 1];

    sieve[0] = false;

    let mut count = 0;
    for i in 0..=limit {
        if sieve[i] {
            let mut temp = i as i32;
            
            let mut curr = Vec::new();

            while temp > 0 {
                curr.insert(0, temp % 10);
                temp /= 10;
            }

            let mut iters = 0;
            let mut results = vec![i as i32];

            loop {
                if curr.len() < 5 {
                    let mut reconstructed = 0;

                    for i in curr.iter().copied().rev() {
                        reconstructed *= 10;
                        reconstructed += i;
                    }
                    
                    results.push(reconstructed);
                }

                let mut rev = reverse(&curr);
                curr = add_two_nums(&mut curr, &mut rev);

                if is_palindrome(&curr) {
                    break;
                }

                iters += 1;
                if iters >= 50 {
                    count += 1;
                    break;
                }
            }

            // println!("{}: {} iters", i, iters);

            if iters < 50 {
                for i in results {
                    sieve[i as usize] = false;
                }
            }
        }
    }

    count
}

fn reverse(x: &Vec<i32>) -> Vec<i32> {
    (*x).iter().copied().rev().collect::<Vec<i32>>()
}

fn is_palindrome(x: &Vec<i32>) -> bool {
    *x == x.iter().copied().rev().collect::<Vec<i32>>()
}

fn add_two_nums(num1: &mut Vec<i32>, num2: &mut Vec<i32>) -> Vec<i32> {
    let mut sum_arr = Vec::new();

    let mut carry = 0;
    while !num1.is_empty() || !num2.is_empty() || carry != 0 {
        let i1 = if num1.is_empty() { 0 } else { num1.pop().unwrap() };
        let i2 = if num2.is_empty() { 0 } else { num2.pop().unwrap() };
        let sum = carry + i1 + i2;

        sum_arr.insert(0, sum % 10);

        carry = sum / 10;
    }

    let last_vec = if !num1.is_empty() { num1 } else { num2 };
    while !last_vec.is_empty() {
        let sum = last_vec.pop().unwrap() + carry;

        sum_arr.insert(0, sum % 10);

        if sum >= 10 {
            carry = 1;
        }
    }

    sum_arr
}