fn main() {
    println!("{}", fibonacci_sequence(1000));
}

fn fibonacci_sequence(digit_count: usize) -> usize {
    let mut num1 = vec![1];
    let mut num2 = vec![1];

    let mut index = 1;
    while num1.len() < digit_count {
        let temp = num2.clone();

        num2 = add_two_nums(num1.clone(), num2.clone());
        num1 = temp;
        index += 1;
    }

    index
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