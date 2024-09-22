fn main() {
    println!("{}", factorial_digit_sum(100));
}

fn factorial_digit_sum(number_to_factorial: i32) -> i32 {
    let mut factorial = vec![1];
    let mut i = 1;

    while i <= number_to_factorial {
        factorial = multiply_big_nums(factorial.clone(), vec![i]);

        i += 1;
    }

    factorial.iter().sum()
}

fn multiply_big_nums(num1: Vec<i32>, mut num2: Vec<i32>) -> Vec<i32> {
    let mut final_num = Vec::new();
    let mut resulting_nums: Vec<Vec<i32>> = Vec::new();

    let mut place = 0;
    while !num2.is_empty() {
        let digit = num2.pop().unwrap();
        place += 1;

        let mut resulting_num = vec![0; place];

        let mut carry = 0;
        for i in (0..num1.len()).rev() {
            let curr_num = num1[i];

            let product = digit * curr_num + carry;

            resulting_num.insert(0, product % 10);
            carry = product / 10;
        }

        if carry > 0 {
            resulting_num.insert(0, carry);
        }

        resulting_nums.push(resulting_num);
    }

    for i in resulting_nums {
        final_num = add_big_nums(final_num, i);
    }

    final_num
}

fn add_big_nums(mut num1: Vec<i32>, mut num2: Vec<i32>) -> Vec<i32> {
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