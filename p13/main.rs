use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let nums = init_num_vec("/Users/aarnavsrivastava/Desktop/_/rust_problems/p13/input.txt")?;
    println!("{}", first_ten_digits_of_large_sum(nums)?);

    Ok(())
}

fn first_ten_digits_of_large_sum(nums: Vec<Vec<i32>>) -> Result<String, Box<dyn Error>> {
    let sum = large_sum(nums)?;

    let mut digits = String::new();

    println!("{:?}", sum);

    for i in 0..10 {
        digits.push((sum[i] as u8 + b'0') as char);
    }

    Ok(digits)
}

fn large_sum(nums: Vec<Vec<i32>>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut sum = nums[0].clone();

    for i in 1..nums.len() {
        sum = add_two_nums(sum, nums[i].clone())?;
    }

    Ok(sum)
}

fn add_two_nums(mut num1: Vec<i32>, mut num2: Vec<i32>) -> Result<Vec<i32>, Box<dyn Error>> {
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

    println!("{:?}", sum_arr);

    Ok(sum_arr)
}

fn init_num_vec(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut nums = vec![vec![0; 50]; 100];

    let mut i = 0;
    for line in reader.lines() {
        let num = line?;

        let mut j = 0;
        for k in num.chars() {
            nums[i][j] = k as i32 - 0x30;

            j += 1
        }

        i += 1;
    }

    Ok(nums)
}