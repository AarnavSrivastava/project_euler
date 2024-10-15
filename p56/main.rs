fn main() {
    // let mut x2 = vec![4, 3, 5, 6];
    // println!("{:?}", multiply_big_nums(vec![2, 3, 1], &mut x2));

    println!("{}", powerful_digit_sum(100));
}

fn powerful_digit_sum(range: i32) -> i32 {
    let mut max = 0;

    for a in 1..=range {
        let mut base = Vec::new();
            
        let mut temp = a;
        while temp > 0 {
            base.insert(0, temp % 10);
            temp /= 10;
        }

        for b in 1..=range {
            let mut result = base.clone();

            // println!("{:?}", result);

            for j in 0..b {
                result = multiply_big_nums(base.clone(), &mut result);
            }

            let res_sum = result.iter().sum();

            if res_sum > max {
                max = res_sum;
            }
        }
    }

    max
}

fn multiply_big_nums(num1: Vec<i32>, num2: &mut Vec<i32>) -> Vec<i32> {
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

    for mut i in resulting_nums {
        final_num = add_two_nums(&mut final_num, &mut i);
    }

    final_num
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