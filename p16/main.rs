fn main() {
    println!("{}", power_digit_sum(1000));
}

fn power_digit_sum(power: i32) -> i32 {
    let mut digits = Vec::new();

    digits.push(1);

    for _i in 0..power {
        digits = add_two_nums(digits.clone(), digits.clone());
    }

    let sum: i32 = digits.iter().sum();

    sum
}

fn add_two_nums(mut num1: Vec<i32>, mut num2: Vec<i32>) -> Vec<i32> {
    let mut sum_arr = Vec::new();

    let mut carry = 0;
    while !num1.is_empty() || !num2.is_empty() || carry != 0 {
        let i1 = if num1.is_empty() { 0 } else { num1.pop().unwrap() };
        let i2 = if num2.is_empty() { 0 } else { num2.pop().unwrap() };
        let sum = carry + i1 + i2;

        sum_arr.insert(0, sum % 10);

        carry = sum / 10;
    }

    let mut last_vec = if !num1.is_empty() { num1 } else { num2 };
    while !last_vec.is_empty() {
        let sum = last_vec.pop().unwrap() + carry;

        sum_arr.insert(0, sum % 10);

        if sum >= 10 {
            carry = 1;
        }
    }

    sum_arr
}